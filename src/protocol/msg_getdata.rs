use std;
extern crate time;
use super::Address;
use ::serialize::{self, Serializable};

#[derive(Debug,Default)]
pub struct GetDataMessage {
   pub addrs : Vec<Address>,
}
impl std::fmt::Display for GetDataMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetData(len={})", self.addrs.len())
   }
}

impl Serializable for GetDataMessage {
   fn get_serialize_size(&self, stype:i32) -> usize {
      self.addrs.get_serialize_size(stype)
   }
   fn serialize(&self, io:&mut std::io::Write, stype:i32) -> serialize::Result {
      self.addrs.serialize(io, stype)
   }
   fn unserialize(&mut self, io:&mut std::io::Read, stype:i32) -> serialize::Result {
      self.addrs.unserialize(io, stype)
   }
}

