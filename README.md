# sherlock-core

A rust project seeking to rewrite [neon-core](https://github.com/NeonGeckoCom/NeonCore)/[mycroft-core](https://github.com/MycroftAI/mycroft-core) in rust. Originally these projects were written in Python; sherlock seeks to rewrite them in faster & safer rust, while maintaining python compatibility. Python compatibility is achieved through [PYO3](https://docs.rs/pyo3/latest/pyo3/) and [maturin](https://github.com/PyO3/maturin).

## Development Progress

- [x] write a log library for sherlock
- [x] make structs/enums for mycroft messages ([docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md))
- [ ] port the message-bus
    - [x] understand mycroft's message-bus
    - [ ] ~understand neon's upgrades to mycroft's message-bus~
    - [x] create message-bus API docs. (Reason: mycroft docs are great but inseficient for my purposes, and Neon's docs are comparatively non-existent)
    - [x] implement in rust
    - [ ] expose rust implementation to python through a library built using maturin & PyO3
        - [ ] make a message object for python
        - [ ] make message send functions, etc
- [x] figure out a way to call out to [mimic3](https://github.com/MycroftAI/mimic3) (mimic3 has an HTTP server, use that for develo?pment, testing, and as a backup.)
    - [x] implement a rust interface for comunications with mimic3 from rust.
        - [x] run the mimic3 shell command and feed it data through its STDIN then read from its STDOUT.
        - [ ] ~send output via the `spoken` message type~
- [ ] make intake server to receive uterances from the outside world
    - [x] function over Websockets
    - [x] conncted to the message-bus
    - [x] take uterances/commands from the outside world.
    - [ ] send back `speak` messages, &:
        - [ ] TTS speech data if requested,
        - [ ] client control byte code (if received from the message-bus)
    - [ ] handles authentication
    - [ ] optionally secured with TLS
        - [x] turn on/off from config file.
- [ ] make skill chooser
    - [ ] make a utility to be used by a python decorator that loads intent files and produces a way to check the users utterance against that file.
<!-- - [ ]  -->

**more to come...**

- [ ] all core components implemented in rust and exposed to python


