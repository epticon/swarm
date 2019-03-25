use crate::alligator::swarm::Message;
use actix::Recipient;

pub(crate) mod uavs;
pub(crate) mod users;

pub(crate) trait ClientTrait<'a, Address = &'a Recipient<Message>> {
    fn address(&'a self) -> Address;

    fn hash(&'a self) -> &'a str;
}
