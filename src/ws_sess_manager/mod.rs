use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler, Message};
use crate::ws_actor::{BroadcastMessage, Connect, DefaultMessage, Disconnect, WebSocket};

/// # Web Socket Session Manager
/// This struct manage session in websocket on a new event
/// it'll be sent broadcast to all active client except for the sender
pub struct WsSessionManager {
    sessions: HashSet<Addr<WebSocket>>,
    pub(crate) last_text: String
}

impl WsSessionManager {
    pub(crate) fn new() -> Self {

        let default_str =
            "{\"ops\":[{\"insert\":\"Write \"},{\"attributes\":{\"underline\":true},\
        \"insert\":\"here\"},{\"insert\":\" some \"},{\"attributes\":{\"bold\":true},\
        \"insert\":\"text\"},{\"insert\":\"!\"}]}";

        Self {
            sessions: HashSet::new(),
            last_text: String::from(default_str)
        }
    }
}

impl Actor for WsSessionManager {
    type Context = Context<Self>;
}



impl Handler<Connect> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("New client connected");
        let addr = msg.addr;
        self.sessions.insert(addr.clone());

        let welcome_text = self.last_text.to_string();
        addr.do_send(DefaultMessage { text: welcome_text });
    }
}

impl Handler<Disconnect> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Client disconnected");
        self.sessions.remove(&msg.addr);
    }
}

impl Handler<BroadcastMessage> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        self.last_text = msg.msg.clone();
        for addr in &self.sessions {
            if *addr != msg.sender {
                let msg_clone = msg.clone();
                addr.do_send(msg_clone);
            }
        }
    }
}