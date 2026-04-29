use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::parser::{Diagnostic, DiagnosticSeverity, IssueKind, SourceLocation, SourceSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct ContextSchema {
    root: ContextShape,
    diagnostics: Vec<Diagnostic>,
}

impl ContextSchema {
    pub fn from_json(schema: Value, source_name: impl Into<String>) -> Self {
        let source_name = source_name.into();
        let mut converter = SchemaConverter {
            source_name,
            diagnostics: Vec::new(),
        };
        let root = converter.convert(&schema, "$");
        Self {
            root,
            diagnostics: converter.diagnostics,
        }
    }

    pub fn root(&self) -> &ContextShape {
        &self.root
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub fn resolve_path(&self, path: &str) -> PathResolution<'_> {
        self.root.resolve_path(path)
    }

    pub fn section_scope(&self, path: &str) -> SectionScope<'_> {
        self.root.section_scope(path)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextShape {
    Object(ObjectShape),
    Array(ArrayShape),
    Scalar {
        kind: ScalarKind,
        enum_values: Vec<Value>,
        default_value: Option<Value>,
    },
    Any,
    Unknown,
    Unsupported,
}

impl ContextShape {
    pub fn resolve_path(&self, path: &str) -> PathResolution<'_> {
        if path == "." {
            return PathResolution::Found {
                shape: self,
                optional: None,
            };
        }

        let mut current = self;
        let mut traversed = Vec::new();
        let mut optional = None;

        for segment in path.split('.') {
            if segment.is_empty() {
                return PathResolution::Missing {
                    missing_path: path.to_owned(),
                    known_fields: Vec::new(),
                };
            }

            match current {
                ContextShape::Object(object) => {
                    traversed.push(segment);
                    let Some(next) = object.properties.get(segment) else {
                        return match object.additional_properties {
                            AdditionalProperties::Permissive => PathResolution::Permissive {
                                through_path: traversed.join("."),
                            },
                            AdditionalProperties::Closed => PathResolution::Missing {
                                missing_path: traversed.join("."),
                                known_fields: object.known_fields(),
                            },
                        };
                    };
                    if optional.is_none() && !object.is_required(segment) {
                        optional = Some(traversed.join("."));
                    }
                    current = next;
                }
                ContextShape::Any | ContextShape::Unknown | ContextShape::Unsupported => {
                    return PathResolution::Permissive {
                        through_path: traversed.join("."),
                    };
                }
                ContextShape::Array(_) | ContextShape::Scalar { .. } => {
                    return PathResolution::InvalidTraversal {
                        traversed_path: traversed.join("."),
                        shape: current,
                    };
                }
            }
        }

        PathResolution::Found {
            shape: current,
            optional,
        }
    }

    pub fn section_scope(&self, path: &str) -> SectionScope<'_> {
        match self.resolve_path(path) {
            PathResolution::Found { shape, optional } => match shape {
                ContextShape::Object(_) => SectionScope::Changed { shape, optional },
                ContextShape::Array(array) => SectionScope::Changed {
                    shape: &array.items,
                    optional,
                },
                ContextShape::Scalar {
                    kind: ScalarKind::Boolean,
                    ..
                } => SectionScope::Current { optional },
                ContextShape::Scalar { .. } => SectionScope::Invalid { shape, optional },
                ContextShape::Any | ContextShape::Unknown | ContextShape::Unsupported => {
                    SectionScope::Current { optional }
                }
            },
            PathResolution::Missing {
                missing_path,
                known_fields,
            } => SectionScope::Missing {
                missing_path,
                known_fields,
            },
            PathResolution::Permissive { through_path } => {
                SectionScope::Permissive { through_path }
            }
            PathResolution::InvalidTraversal {
                traversed_path,
                shape,
            } => SectionScope::InvalidTraversal {
                traversed_path,
                shape,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectShape {
    pub properties: BTreeMap<String, ContextShape>,
    required: BTreeSet<String>,
    pub additional_properties: AdditionalProperties,
}

impl ObjectShape {
    pub fn is_required(&self, property: &str) -> bool {
        self.required.contains(property)
    }

    pub fn known_fields(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayShape {
    pub items: Box<ContextShape>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditionalProperties {
    Permissive,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarKind {
    String,
    Number,
    Integer,
    Boolean,
    Null,
}

impl ScalarKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Null => "null",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathResolution<'a> {
    Found {
        shape: &'a ContextShape,
        optional: Option<String>,
    },
    Missing {
        missing_path: String,
        known_fields: Vec<String>,
    },
    Permissive {
        through_path: String,
    },
    InvalidTraversal {
        traversed_path: String,
        shape: &'a ContextShape,
    },
}

impl<'a> PathResolution<'a> {
    pub fn shape(&self) -> Option<&'a ContextShape> {
        match self {
            Self::Found { shape, .. } | Self::InvalidTraversal { shape, .. } => Some(shape),
            Self::Missing { .. } | Self::Permissive { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionScope<'a> {
    Changed {
        shape: &'a ContextShape,
        optional: Option<String>,
    },
    Current {
        optional: Option<String>,
    },
    Invalid {
        shape: &'a ContextShape,
        optional: Option<String>,
    },
    Missing {
        missing_path: String,
        known_fields: Vec<String>,
    },
    Permissive {
        through_path: String,
    },
    InvalidTraversal {
        traversed_path: String,
        shape: &'a ContextShape,
    },
}

struct SchemaConverter {
    source_name: String,
    diagnostics: Vec<Diagnostic>,
}

impl SchemaConverter {
    fn convert(&mut self, schema: &Value, path: &str) -> ContextShape {
        let Some(object) = schema.as_object() else {
            self.warn(format!("schema value at `{path}` must be an object"));
            return ContextShape::Unknown;
        };

        self.warn_for_unsupported_constructs(object, path);

        let type_name = match object.get("type") {
            Some(Value::String(type_name)) => Some(type_name.as_str()),
            Some(value) => {
                self.warn(format!(
                    "schema keyword `{path}.type` must be a supported string, found {}",
                    value_kind(value)
                ));
                return ContextShape::Unknown;
            }
            None => None,
        };

        match type_name {
            Some("object") => self.convert_object(object, path),
            Some("array") => self.convert_array(object, path),
            Some("string") => self.convert_scalar(object, ScalarKind::String),
            Some("number") => self.convert_scalar(object, ScalarKind::Number),
            Some("integer") => self.convert_scalar(object, ScalarKind::Integer),
            Some("boolean") => self.convert_scalar(object, ScalarKind::Boolean),
            Some("null") => self.convert_scalar(object, ScalarKind::Null),
            Some(other) => {
                self.warn(format!(
                    "unsupported schema type `{other}` at `{path}.type`"
                ));
                ContextShape::Unsupported
            }
            None if object.contains_key("properties") => self.convert_object(object, path),
            None if object.contains_key("items") => self.convert_array(object, path),
            None if object.contains_key("enum") => {
                self.warn(format!(
                    "schema keyword `{path}.enum` requires a scalar `type` to preserve allowed values"
                ));
                ContextShape::Unknown
            }
            None if object.is_empty() => ContextShape::Any,
            None => ContextShape::Any,
        }
    }

    fn convert_object(
        &mut self,
        object: &serde_json::Map<String, Value>,
        path: &str,
    ) -> ContextShape {
        let mut properties = BTreeMap::new();
        if let Some(value) = object.get("properties") {
            if let Some(property_map) = value.as_object() {
                for (name, property_schema) in property_map {
                    properties.insert(
                        name.clone(),
                        self.convert(property_schema, &format!("{path}.properties.{name}")),
                    );
                }
            } else {
                self.warn(format!(
                    "schema keyword `{path}.properties` must be an object, found {}",
                    value_kind(value)
                ));
            }
        }

        let mut required = BTreeSet::new();
        if let Some(value) = object.get("required") {
            if let Some(items) = value.as_array() {
                for item in items {
                    if let Some(name) = item.as_str() {
                        required.insert(name.to_owned());
                    } else {
                        self.warn(format!(
                            "schema keyword `{path}.required` must contain only strings"
                        ));
                    }
                }
            } else {
                self.warn(format!(
                    "schema keyword `{path}.required` must be an array, found {}",
                    value_kind(value)
                ));
            }
        }

        ContextShape::Object(ObjectShape {
            properties,
            required,
            additional_properties: self.additional_properties(object, path),
        })
    }

    fn convert_array(
        &mut self,
        object: &serde_json::Map<String, Value>,
        path: &str,
    ) -> ContextShape {
        let items = match object.get("items") {
            Some(value) if value.is_object() => self.convert(value, &format!("{path}.items")),
            Some(value) => {
                self.warn(format!(
                    "schema keyword `{path}.items` must be an object, found {}",
                    value_kind(value)
                ));
                ContextShape::Unknown
            }
            None => ContextShape::Any,
        };

        ContextShape::Array(ArrayShape {
            items: Box::new(items),
        })
    }

    fn convert_scalar(
        &mut self,
        object: &serde_json::Map<String, Value>,
        kind: ScalarKind,
    ) -> ContextShape {
        let enum_values = match object.get("enum") {
            Some(Value::Array(values)) => values.clone(),
            Some(value) => {
                self.warn(format!(
                    "schema keyword `enum` must be an array, found {}",
                    value_kind(value)
                ));
                Vec::new()
            }
            None => Vec::new(),
        };

        ContextShape::Scalar {
            kind,
            enum_values,
            default_value: object.get("default").cloned(),
        }
    }

    fn additional_properties(
        &mut self,
        object: &serde_json::Map<String, Value>,
        path: &str,
    ) -> AdditionalProperties {
        match object.get("additionalProperties") {
            Some(Value::Bool(false)) => AdditionalProperties::Closed,
            Some(Value::Bool(true)) | None => AdditionalProperties::Permissive,
            Some(value) if value.is_object() => {
                self.warn(format!(
                    "schema-valued `{path}.additionalProperties` is unsupported"
                ));
                AdditionalProperties::Permissive
            }
            Some(value) => {
                self.warn(format!(
                    "schema keyword `{path}.additionalProperties` must be boolean, found {}",
                    value_kind(value)
                ));
                AdditionalProperties::Permissive
            }
        }
    }

    fn warn_for_unsupported_constructs(
        &mut self,
        object: &serde_json::Map<String, Value>,
        path: &str,
    ) {
        for keyword in [
            "$ref",
            "$defs",
            "definitions",
            "oneOf",
            "anyOf",
            "allOf",
            "patternProperties",
        ] {
            if object.contains_key(keyword) {
                self.warn(format!("unsupported schema keyword `{path}.{keyword}`"));
            }
        }
    }

    fn warn(&mut self, message: String) {
        self.diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            issue: IssueKind::SchemaInputError,
            source_name: self.source_name.clone(),
            location: SourceLocation { line: 1, column: 1 },
            span: SourceSpan::new(0, 0),
            message,
        });
    }
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
