# sherlock-core

A rust project seeking to rewrite [neon-core](https://github.com/NeonGeckoCom/NeonCore)/[mycroft-core](https://github.com/MycroftAI/mycroft-core) in rust. Originally these projects were written in Python; sherlock seeks to rewrite them in faster & safer rust, while maintaining python compatibility. Python compatibility is acheived through [PYO3](https://docs.rs/pyo3/latest/pyo3/) and [maturin](https://github.com/PyO3/maturin).

## Developement Progress

- [x] write a log library for sherlock
- [ ] make structs/enums for mycroft messages ([docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md))
- [ ] port the message-bus
    - [x] understand mycroft's message-bus
    - [ ] understand neon's upgrades to mycroft's message-bus
    - [ ] create message-bus API docs. (Reason: mycroft docs are great but inseficient for my purposes, and Neon's docs are comparitively non-existent)
    - [ ] implement in rust
    - [ ] expose rust implementation to python through a library built using maturin & PyO3
- [ ] figure out a way to call out to [mimic3](https://github.com/MycroftAI/mimic3) (mimic3 has an HTTP server, use that for developement, testing, and as a backup.)
    - [ ] implement in a rust interface for comunications with mimic3 from rust.

**more to come...**

- [ ] all core components implemented in rust and exposed to python


