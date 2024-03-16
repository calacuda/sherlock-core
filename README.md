# sherlock-core

A rust project seeking to rewrite [neon-core](https://github.com/NeonGeckoCom/NeonCore)/[mycroft-core](https://github.com/MycroftAI/mycroft-core) in rust. Originally these projects were written in Python; sherlock seeks to rewrite them in faster & safer rust, while maintaining python compatibility. Python compatibility is acheived through [PYO3](https://docs.rs/pyo3/latest/pyo3/) and [maturin](https://github.com/PyO3/maturin).

## Developement Progress

- [x] write a log library for sherlock
- [x] make structs/enums for mycroft messages ([docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md))
- [ ] port the message-bus
    - [x] understand mycroft's message-bus
    - [ ] ~understand neon's upgrades to mycroft's message-bus~
    - [x] create message-bus API docs. (Reason: mycroft docs are great but inseficient for my purposes, and Neon's docs are comparitively non-existent)
    - [x] implement in rust
    - [ ] expose rust implementation to python through a library built using maturin & PyO3
        - [ ] make a message object for python
        - [ ] make message send functions, etc
- [ ] figure out a way to call out to [mimic3](https://github.com/MycroftAI/mimic3) (mimic3 has an HTTP server, use that for developement, testing, and as a backup.)
    - [ ] implement a rust interface for comunications with mimic3 from rust.
- [ ] make intake server to recieve uterances from the outside world
    - [ ] function over Websockets
    - [ ] conncted to the message-bus
    - [ ] take uterances/commands from the outside world.
    - [ ] send back `speak` messages, &:
        - [ ] TTS speach data if requested,
        - [ ] client control byte code (if recieved from the message-bus)
    - [ ] handles authentication
    - [ ] optionally secured with TLS
        - [ ] turn on/off from config file.
- [ ] make skill chooser
    - [ ] make a utility to be used by a python decorator that loads intent files and produces a way to check the users utterance against that file.
<!-- - [ ]  -->

**more to come...**

- [ ] all core components implemented in rust and exposed to python


