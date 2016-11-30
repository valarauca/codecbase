use super::codec;
use super::codec::{Codec, Reader};
use std::borrow::Cow;

/// An externally length'd payload
#[derive(Debug, Clone, PartialEq)]
pub struct Payload(pub Vec<u8>);

impl Codec for Payload {
  fn encode(&self, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&self.0);
  }

  fn read(r: &mut Reader) -> Option<Payload> {
    Some(Payload(r.rest().to_vec()))
  }
}

impl Payload {
  pub fn new(bytes: Vec<u8>) -> Payload {
    Payload(bytes)
  }

  pub fn empty() -> Payload {
    Payload::new(Vec::with_capacity(2048))
  }
  
  pub fn from_slice(data: &[u8]) -> Payload {
    let mut v = Vec::with_capacity(data.len()+5);
    v.extend_from_slice(data);
    Payload(v)
  }
  pub fn len(&self) -> usize { self.0.len() }
}

/// An arbitrary, unknown-content, u24-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
pub struct PayloadU24(pub Vec<u8>);

impl PayloadU24 {
  pub fn new(bytes: Vec<u8>) -> PayloadU24 {
    PayloadU24(bytes)
  }

  pub fn len(&self) -> usize { self.0.len() }
}

impl Codec for PayloadU24 {
  fn encode(&self, bytes: &mut Vec<u8>) {
    codec::encode_u24(self.0.len() as u32, bytes);
    bytes.extend_from_slice(&self.0);
  }

  fn read(r: &mut Reader) -> Option<PayloadU24> {
    let s = try_ret!(r.u24_encoded_slice());
    Some(PayloadU24::new(s.to_vec()))
  }
}

/// An arbitrary, unknown-content, u16-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
pub struct PayloadU16(pub Vec<u8>);

impl PayloadU16 {
  pub fn new(bytes: Vec<u8>) -> PayloadU16 {
    PayloadU16(bytes)
  }

  pub fn len(&self) -> usize { self.0.len() }
}

impl Codec for PayloadU16 {
  fn encode(&self, bytes: &mut Vec<u8>) {
    codec::encode_u16(self.0.len() as u16, bytes);
    bytes.extend_from_slice(&self.0);
  }

  fn read(r: &mut Reader) -> Option<PayloadU16> {
    let len = try_ret!(r.read_u16());
    let sub = try_ret!(r.sub(len));
    let body = sub.rest().to_vec();
    Some(PayloadU16(body))
  }
}

/// An arbitrary, unknown-content, u8-length-prefixed payload
#[derive(Debug, Clone, PartialEq)]
pub struct PayloadU8<'a>(pub Cow<'a, [u8]>);

impl<'a> PayloadU8<'a> {
  
  #[inline]
  pub fn from_slice<'b>(bytes: &'b [u8]) -> PayloadU8<'b> {
    PayloadU8(Cow::Borrowed(bytes))
  }
  
  pub fn new(bytes: Vec<u8>) -> Self {
    PayloadU8(Cow::Owned(bytes))
  }

  pub fn len(&self) -> usize { self.0.len() }
}

/*
impl<'a> Codec for PayloadU8<'a> {
  fn encode(&self, bytes: &mut Vec<u8>) {
    codec::encode_u8(self.0.len() as u8, bytes);
    bytes.extend_from_slice(self.0.as_ref());
  }

  fn read<'b,'c>(r: &'b mut Reader<'b>) -> Option<PayloadU8<'c>> {
    let slice = match r.u8_encoded_slice() {
      Option::None => return None,
      Option::Some(x) => x,
    };
    Some(PayloadU8::from_slice(slice))
  }
}
*/
