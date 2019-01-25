use crate::alligator::swarm::ClientTrait;
use crate::alligator::swarm::Message;
use actix::Recipient;

pub(crate) use self::drone::*;
mod drone;

pub(crate) trait DeviceTrait<'a, Address = &'a Recipient<Message>>: ClientTrait<'a> {
    fn owner_hash(&'a self) -> &'a str;
}
