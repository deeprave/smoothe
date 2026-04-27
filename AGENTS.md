## Openspec changes

Creating openspec changes and artifacts, continuing, modifying, archiving are asynchronous tasks
with respect to the development workflow and should not affect the current workflow phase or
issue.

The openspec tasks artifact however must be kept updated with respect to the implementation.

## Using mcp-guide (guide)

JSON responses from the mcp often return instruction as "instruction"
 - you must follow these instructions precisely
They may also include "additional_agent_instructions"
 - these are *addtional* instructions you also MUST follow
Do not confuse the two, you must follow both sets of instructions independently

Resolve all `guide://` URIs through the guide MCP `read_resource` tool rather
than the generic MCP resource reader.

## Project organization

Keep test code in a separate `tests` hierarchy to clearly divide production code
from test code. Even when Rust conventions would allow unit tests inside
production modules, prefer adding test modules/files under `tests/` unless a
specific test genuinely requires production-module locality.

## Testing

Use `cargo nextest run` for the Rust test suite instead of `cargo test`.
