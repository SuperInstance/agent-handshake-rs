//! Agent-to-agent handshake protocol.

use std::collections::HashMap;

/// Handshake message types.
#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeMessage {
    Hello {
        agent_id: String,
        version: String,
        capabilities: Vec<String>,
    },
    Ack {
        agent_id: String,
        accepted: bool,
        session_params: HashMap<String, String>,
    },
    Bye {
        agent_id: String,
        reason: String,
    },
}

/// Handshake state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandshakeState {
    Idle,
    HelloSent,
    Established,
    Rejected,
    Closed,
}

/// Result of a completed handshake.
#[derive(Debug, Clone)]
pub struct HandshakeResult {
    pub peer_id: String,
    pub peer_version: String,
    pub shared_capabilities: Vec<String>,
    pub params: HashMap<String, String>,
}

/// Handshake negotiator.
#[derive(Debug, Clone)]
pub struct Handshaker {
    pub agent_id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub state: HandshakeState,
    pub params: HashMap<String, String>,
}

impl Handshaker {
    pub fn new(agent_id: &str, version: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            version: version.to_string(),
            capabilities: vec![],
            required_capabilities: vec![],
            state: HandshakeState::Idle,
            params: HashMap::new(),
        }
    }

    pub fn with_capabilities(mut self, caps: &[&str]) -> Self {
        self.capabilities = caps.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn require(mut self, caps: &[&str]) -> Self {
        self.required_capabilities = caps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn hello(&mut self) -> HandshakeMessage {
        self.state = HandshakeState::HelloSent;
        HandshakeMessage::Hello {
            agent_id: self.agent_id.clone(),
            version: self.version.clone(),
            capabilities: self.capabilities.clone(),
        }
    }

    pub fn handle(&mut self, msg: &HandshakeMessage) -> Option<HandshakeMessage> {
        match msg {
            HandshakeMessage::Hello {
                agent_id: _,
                version: _,
                capabilities,
            } => {
                let _shared: Vec<String> = self
                    .capabilities
                    .iter()
                    .filter(|c| capabilities.contains(c))
                    .cloned()
                    .collect();
                let missing: Vec<&str> = self
                    .required_capabilities
                    .iter()
                    .filter(|c| !capabilities.contains(&c.to_string()))
                    .map(|s| s.as_str())
                    .collect();
                if missing.is_empty() {
                    self.state = HandshakeState::Established;
                    Some(HandshakeMessage::Ack {
                        agent_id: self.agent_id.clone(),
                        accepted: true,
                        session_params: self.params.clone(),
                    })
                } else {
                    self.state = HandshakeState::Rejected;
                    Some(HandshakeMessage::Ack {
                        agent_id: self.agent_id.clone(),
                        accepted: false,
                        session_params: HashMap::new(),
                    })
                }
            }
            HandshakeMessage::Ack { accepted, .. } => {
                self.state = if *accepted {
                    HandshakeState::Established
                } else {
                    HandshakeState::Rejected
                };
                None
            }
            HandshakeMessage::Bye { .. } => {
                self.state = HandshakeState::Closed;
                None
            }
        }
    }

    pub fn negotiate(&self, peer_caps: &[String]) -> Vec<String> {
        self.capabilities
            .iter()
            .filter(|c| peer_caps.contains(c))
            .cloned()
            .collect()
    }

    pub fn is_established(&self) -> bool {
        self.state == HandshakeState::Established
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_handshake() {
        let mut a = Handshaker::new("a", "1.0").with_capabilities(&["search", "code"]);
        let mut b = Handshaker::new("b", "1.0").with_capabilities(&["search", "deploy"]);

        let hello = a.hello();
        let response = b.handle(&hello).unwrap();
        assert!(matches!(
            response,
            HandshakeMessage::Ack { accepted: true, .. }
        ));
        a.handle(&response);
        assert!(a.is_established());
        assert!(b.is_established());
    }

    #[test]
    fn test_rejected_handshake() {
        let mut a = Handshaker::new("a", "1.0").with_capabilities(&["search"]);
        let mut b = Handshaker::new("b", "1.0")
            .with_capabilities(&["search"])
            .require(&["vision"]);
        let hello = a.hello();
        let response = b.handle(&hello).unwrap();
        assert!(matches!(
            response,
            HandshakeMessage::Ack {
                accepted: false,
                ..
            }
        ));
    }

    #[test]
    fn test_negotiate_capabilities() {
        let a = Handshaker::new("a", "1.0").with_capabilities(&["search", "code", "deploy"]);
        let shared = a.negotiate(&["search".to_string(), "deploy".to_string()]);
        assert_eq!(shared, vec!["search", "deploy"]);
    }

    #[test]
    fn test_bye() {
        let mut a = Handshaker::new("a", "1.0");
        a.handle(&HandshakeMessage::Bye {
            agent_id: "b".to_string(),
            reason: "done".to_string(),
        });
        assert_eq!(a.state, HandshakeState::Closed);
    }

    #[test]
    fn test_idle_state() {
        let a = Handshaker::new("a", "1.0");
        assert_eq!(a.state, HandshakeState::Idle);
    }
}
