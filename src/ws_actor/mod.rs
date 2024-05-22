use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;
use crate::ws_sess_manager::WsSessionManager;

/// Struct to connect new WS client
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>,
}

/// Struct to disconnect new WS client
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub(crate) msg: String,
    pub(crate) sender: Addr<WebSocket>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DefaultMessage {
    pub text: String,
}

impl Handler<DefaultMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: DefaultMessage, ctx: &mut Self::Context) {
        ctx.text(msg.text);  // Invia il testo al client
    }
}


pub struct WebSocket {
    pub(crate) manager: Addr<WsSessionManager>,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.do_send(Connect {
            addr: ctx.address(),
        });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.manager.do_send(Disconnect {
            addr: ctx.address(),
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            self.manager.do_send(BroadcastMessage {
                msg: text.to_string(),
                sender: ctx.address(),
            });
        }
    }
}

impl Handler<BroadcastMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.msg);
    }
}
