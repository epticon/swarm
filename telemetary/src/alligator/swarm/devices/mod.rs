use crate::alligator::swarm::Message;
use actix::Recipient;

pub(crate) mod drone;

pub(crate) trait DeviceTrait<'a, Address = &'a Recipient<Message>> {
  fn address(&'a self) -> Address;
  fn owner_hash(&'a self) -> &'a str;
  fn hash(&'a self) -> &'a str;
}
