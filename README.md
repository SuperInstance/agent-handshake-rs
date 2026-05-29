# agent-handshake-rs

Rust port of [agent-handshake](https://github.com/SuperInstance/agent-handshake) — agent-to-agent handshake protocol.

## Features

- **Hello/Ack/Bye protocol**: lightweight capability negotiation
- **Required capabilities**: reject handshakes that don't meet requirements
- **Shared capability discovery**: find intersection of agent capabilities
- **State machine**: Idle → HelloSent → Established / Rejected / Closed

## Usage

```rust
use agent_handshake::{Handshaker, HandshakeMessage};

let mut alice = Handshaker::new("alice", "1.0")
    .with_capabilities(&["search", "code", "deploy"]);
let mut bob = Handshaker::new("bob", "1.0")
    .with_capabilities(&["search", "deploy"])
    .require(&["search"]);

// Alice initiates
let hello = alice.hello();
let ack = bob.handle(&hello).unwrap(); // accepted
alice.handle(&ack);

assert!(alice.is_established());
assert!(bob.is_established());

// Negotiate shared capabilities
let shared = alice.negotiate(&["search".to_string(), "deploy".to_string()]);
assert_eq!(shared, vec!["search", "deploy"]);
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
