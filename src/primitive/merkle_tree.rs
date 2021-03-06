use std;
extern crate bit_vec;
use super::{UInt256};
use ::serialize::{self, Serializable};

fn reverse_u8(x:u8) -> u8 {
   let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
   let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
   let x:u8 = (x << 4) | (x >> 4);
   x
}

#[derive(Debug,Default,Clone)]
pub struct PartialMerkleTree {
   pub n_transactions: u32,
   pub bits: bit_vec::BitVec,
   pub hashes: Vec<UInt256>,
}

impl std::fmt::Display for PartialMerkleTree {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "PartialMerkleTree(n={}, bits={:?}, hash={:?})", self.n_transactions, self.bits, self.hashes)
   }
}

impl Serializable for PartialMerkleTree {
   fn get_serialize_size(&self, _ser:&serialize::SerializeParam) -> usize {
      4 + (self.bits.len()+7)/8 + 32*self.hashes.len()
   }
   fn serialize(&self, io:&mut std::io::Write, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.n_transactions.serialize(io, ser));
      {
         let mut bytes = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8(*byte);
         }
         r += try!(bytes.serialize(io, ser));
      }         
      r += try!(self.hashes.serialize(io, ser));
      Ok(r)
   }
   fn deserialize(&mut self, io:&mut std::io::Read, ser:&serialize::SerializeParam) -> serialize::Result {
      let mut r:usize = 0;
      r += try!(self.n_transactions.deserialize(io, ser));
      {
         let mut bytes:Vec<u8> = Vec::new();
         r += try!(bytes.deserialize(io, ser));

         for byte in &mut bytes {
            *byte = reverse_u8(*byte);
         }
         self.bits = bit_vec::BitVec::from_bytes(bytes.as_slice());
      }         
      r += try!(self.hashes.deserialize(io, ser));
      Ok(r)
   }
}

