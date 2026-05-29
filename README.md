# agent-handshake-rs

Agent-to-agent capability negotiation — a lightweight Hello/Ack/Bye protocol for establishing communication, discovering shared capabilities, and enforcing requirements.

## What This Gives You

- **Hello/Ack/Bye message protocol**: structured handshake with agent ID, version, and capabilities
- **Required capabilities**: reject handshakes that don't meet your minimum requirements
- **Shared capability discovery**: automatically find the intersection of what both agents support
- **State machine**: Idle → HelloSent → Established / Rejected / Cleanly Closed
- **Session parameters**: exchange key-value configuration during handshake

## Quick Start

```rust
use agent_handshake::{Handshaker, HandshakeMessage};

let mut alice = Handshaker::new("alice", "1.0")
    .with_capabilities(&["search", "code", "deploy"]);
let mut bob = Handshaker::new("bob", "1.0")
    .with_capabilities(&["search", "deploy"])
    .require(&["search"]);  // bob won't talk to agents without "search"

// Alice initiates
let hello = alice.hello();  // state: HelloSent
let ack = bob.handle(&hello).unwrap();  // accepted — bob has "search"
alice.handle(&ack);  // state: Established

assert!(alice.is_established());
assert!(bob.is_established());

// Negotiate what both agents can do together
let shared = alice.shared_capabilities();
// → ["search", "deploy"]

// Clean shutdown
let bye = alice.bye("done");
bob.handle(&bye);  // state: Closed
```

## API Reference

### Handshaker

```rust
impl Handshaker {
    pub fn new(agent_id: &str, version: &str) -> Self;
    pub fn with_capabilities(self, caps: &[&str]) -> Self;
    pub fn require(self, caps: &[&str]) -> Self;
    pub fn hello(&mut self) -> HandshakeMessage;
    pub fn bye(&mut self, reason: &str) -> HandshakeMessage;
    pub fn handle(&mut self, msg: &HandshakeMessage) -> Result<HandshakeMessage, String>;
    pub fn is_established(&self) -> bool;
    pub fn shared_capabilities(&self) -> Vec<String>;
    pub fn result(&self) -> Option<HandshakeResult>;
}
```

### HandshakeMessage

```rust
pub enum HandshakeMessage {
    Hello { agent_id, version, capabilities },
    Ack   { agent_id, accepted, session_params },
    Bye   { agent_id, reason },
}
```

### HandshakeState

`Idle` → `HelloSent` → `Established` | `Rejected` | `Closed`

## How It Fits

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem. Typically the first thing that happens before:

- **agent-identity-rs** — verify identity after handshake
- **agent-manifest-rs** — capability descriptors that get exchanged during handshake
- **agent-shadow-rs** — shadow recording begins after handshake establishes the session

## Testing

**5 tests** covering full handshake flow, capability negotiation, required capability rejection, bye/closed transitions, and duplicate handling.

## Installation

```toml
# Cargo.toml
[dependencies]
agent-handshake = { git = "https://github.com/SuperInstance/agent-handshake-rs" }
```

Requires Rust 2021 edition. No external dependencies.
