use anyhow::Result;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    identity::Keypair,
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour, Swarm,
};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::Messages;

pub struct CustomSwarm<BlockchainBehaviour: libp2p::swarm::NetworkBehaviour> {
    pub swarm: Swarm<BlockchainBehaviour>,
    pub msg_sender: mpsc::UnboundedSender<Messages>,
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct BlockchainBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
}

pub enum MyBehaviourEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<GossipsubEvent> for MyBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        MyBehaviourEvent::Gossipsub(event)
    }
}

impl From<MdnsEvent> for MyBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        MyBehaviourEvent::Mdns(event)
    }
}

impl BlockchainBehaviour {
    pub async fn new(
        key_pair: Keypair,
        config: GossipsubConfig,
        // msg_sender: mpsc::UnboundedSender<Messages>,
    ) -> Result<Self> {
        Ok(Self {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(key_pair), config).unwrap(),
            mdns: Mdns::new(Default::default()).await?,
        })
    }
}

impl CustomSwarm<BlockchainBehaviour> {
    pub fn handle_event(&mut self, event: MyBehaviourEvent) {
        match event {
            MyBehaviourEvent::Gossipsub(event) => match event {
                GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                } => {
                    info!("Got message with id: {} from peer: {:?}", id, peer_id);
                    let msg: Messages = serde_json::from_slice(&message.data).unwrap();
                    if let Err(e) = self.msg_sender.send(msg) {
                        error!("error sending messages via channel, {}", e);
                    }
                }
                GossipsubEvent::Subscribed { peer_id, topic } => {
                    info!("Subscribe topic: {} with id: {}", topic, peer_id);
                }
                GossipsubEvent::Unsubscribed { peer_id, topic } => {
                    info!("Unsubscribe topic: {} with id: {}", topic, peer_id);
                }
                GossipsubEvent::GossipsubNotSupported { peer_id } => {
                    info!("Not supported Gossip sub with id: {}", peer_id);
                }
            },

            MyBehaviourEvent::Mdns(event) => match event {
                MdnsEvent::Discovered(list) => {
                    for (id, addr) in list {
                        println!("Got peer: {} with addr {}", &id, &addr);
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&id);
                    }
                }
                MdnsEvent::Expired(list) => {
                    for (id, addr) in list {
                        println!("Removed peer: {} with addr {}", &id, &addr);
                        self.swarm
                            .behaviour_mut()
                            .gossipsub
                            .remove_explicit_peer(&id);
                    }
                }
            },
        }
    }
}
