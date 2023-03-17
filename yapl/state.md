# Development state

Some notes for history...

## Ongoing

### Overview

Goal: determine basics of compiler architecture.

Initial version will be divided into five modules:

- parser: temporal frontend,

- ast: provide macros-like expressiveness,

- core: placeholder for now, will contains most high-level processes,

- dfg: prune out (most?) compile-time data,

- cfg: simplifying types by using normal graph opposit to dag in previous steps.

Backend will be provided by other crates.

### Implemented parts

Parser: initial version is implemented. Current version:

- is expressive enough to work with,

- isn't stable, all features can be changed,

- has basic diagnostic, but,

- wasn't tested well.
