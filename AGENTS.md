## Using mcp-guide (guide)

JSON responses from the mcp often return instruction as "instruction"
 - you must follow these instructions precisely
They may also include "additional_agent_instructions"
 - these are *addtional* instructions you also MUST follow
Do not confuse the two, you must follow both sets of instructions independently

## Project organization

Keep test code in a separate `tests` hierarchy to clearly divide production code
from test code. Even when Rust conventions would allow unit tests inside
production modules, prefer adding test modules/files under `tests/` unless a
specific test genuinely requires production-module locality.

## Testing

Use `cargo nextest run` for the Rust test suite instead of `cargo test`.
