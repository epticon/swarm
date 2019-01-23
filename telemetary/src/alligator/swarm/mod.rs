use actix::prelude::{Actor, Context, Message as ActixMessage};

mod connect;
mod disconnect;
pub(crate) use self::connect::*;
pub(crate) use self::disconnect::*;

#[derive(ActixMessage)]
pub struct Message(pub String);

pub(crate) struct Swarm;

impl Actor for Swarm {
    type Context = Context<Self>;
}

impl Default for Swarm {
    fn default() -> Self {
        Swarm
    }
}
