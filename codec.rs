use std::fmt::Debug;
use std::borrow::Cow;


///Ensure the COW borrow mutating is done.
#[inline(always)]
pub fn extend(x: &[u8], bytes: &mut Vec<u8>) {
  bytes.extend_from_slice(x)
}

macro_rules! payloadtraits {
  ($type_with_lifetime: ty,$type_for_building:expr) => (
  impl<'a> $type_with_lifetime {
    pub fn new(bytes: Vec<u8>) -> Self {
      $type_for_building(Cow::Owned(bytes))
    }
    pub fn from_slice(data: &'a [u8]) -> Self {
      $type_for_building(Cow::Borrowed(data))
    }
    #[inline(always)]
    pub fn len(&self) ->  usize {
      self.0.len()
    }
  }
)    
}

#[derive(Debug)]
pub struct PayloadU8<'a>(pub Cow<'a,[u8]>);
payloadtraits!(PayloadU8<'a>,PayloadU8);
impl<'a> Codec for PayloadU8<'a> {
  fn encode(&self, bytes: &mut Vec<u8>) {
    encode_u8(self.len() as u8, bytes);
    extend(&self.0,bytes);
  }
  fn read<'b>(r: &mut Reader<'b>) -> Option<PayloadU8<'b>> {
     r.u8_payload()
  }
}
#[derive(Debug)]
pub struct PayloadU16<'a>(pub Cow<'a,[u8]>);
#[derive(Debug)]
pub struct PayloadU24<'a>(pub Cow<'a,[u8]>);
#[derive(Debug)]
pub struct PayloadU32<'a>(pub Cow<'a,[u8]>);
#[derive(Debug)]
pub struct PayloadU64<'a>(pub Cow<'a,[u8]>);


               
               
pub struct Payload<'a>(pub Cow<'a,[u8]>);




///Reader holds a borrowed buffer. It uses this borrow to hold several
///slices of different length, these slices are encoded internally via
///length prefixes. The programmer must remember what order the prefixes
///are stored.
pub struct Reader<'a> {
  buf: &'a [u8],
  offs: usize
}

impl<'a> Reader<'a> {
  
  ///Build a new Reader by borrowing a slice
  pub fn init(bytes: &'a [u8]) -> Reader<'a> {
    Reader { buf: bytes, offs: 0 }
  }

  ///Return all date remaining in the buffer
  pub fn rest(&self) -> &'a [u8] {
    &self.buf[self.offs ..]
  }

  ///Take len amount of data
  pub fn take(&mut self, len: usize) -> Option<&'a [u8]> {
    if self.left() < len {
      return None
    }
    let current = self.offs;
    self.offs += len;
    Some(&self.buf[current .. current + len])
  }

  ///Check if any data remains in the structure
  pub fn any_left(&self) -> bool {
    self.offs < self.buf.len()
  }

  ///get lenght of remaining data
  pub fn left(&self) -> usize {
    self.buf.len() - self.offs
  }

  ///get consumed data
  pub fn used(&self) -> usize {
    self.offs
  }

  ///Make a reader over len which points to THIS reader's buffer
  pub fn sub(&mut self, len: usize) -> Option<Reader<'a>> {
    self.take(len).and_then(|bytes| Some(Reader::init(bytes)))
  }

  ///decode a u8 length from the current offset
  pub fn read_u8(&mut self) -> Option<usize> {
    if self.left() < 1 {
      return None;  
    }
    let arg: u8 = self.buf[self.offs].clone();
    self.offs += 1;
    Some(arg as usize)
  }

  ///decode a u16 length at the current offset
  pub fn read_u16(&mut self) -> Option<usize> {
    if self.left() < 2 {
        return None;
    }
    let arg0 = self.buf[self.offs].clone() as u16;
    let arg1 = self.buf[self.offs+1].clone() as u16;
    let arg0 = arg0 << 8;
    self.offs += 2;
    let ret = arg0|arg1;
    Some(ret as usize)
  }

  ///decode a u24 length at the current offset
  pub fn read_u24(&mut self) -> Option<usize> {
    if self.left() < 3 {
        return None;
    }
    let arg0 = self.buf[self.offs].clone() as u32;
    let arg1 = self.buf[self.offs+1].clone() as u32;
    let arg2 = self.buf[self.offs+2].clone() as u32;
    let arg0 = arg0 << 16;
    let arg1 = arg1 << 8;
    self.offs += 3;
    let ret = arg0 | arg1 | arg2;
    Some(ret as usize)
  }

  ///decode a u32 length at the current offset
  pub fn read_u32(&mut self) -> Option<usize> {
    if self.left() < 4 {
        return None;
    }
    let arg0 = self.buf[self.offs].clone() as u32;
    let arg1 = self.buf[self.offs+1].clone() as u32;
    let arg2 = self.buf[self.offs+2].clone() as u32;
    let arg3 = self.buf[self.offs+3].clone() as u32;
    let arg0 = arg0 << 24;
    let arg1 = arg1 << 16;
    let arg2 = arg2 << 8;
    self.offs += 4;
    let ret = arg0 | arg1 | arg2 | arg3;
    Some(ret as usize)
  }

  ///decode a u64 length at the current offset
  pub fn read_u64(&mut self) -> Option<usize> {
    if self.left() < 8 {
        return None;
    }
    let arg0 = self.buf[self.offs].clone() as u64;
    let arg1 = self.buf[self.offs+1].clone() as u64;
    let arg2 = self.buf[self.offs+2].clone() as u64;
    let arg3 = self.buf[self.offs+3].clone() as u64;
    let arg4 = self.buf[self.offs+4].clone() as u64;
    let arg5 = self.buf[self.offs+5].clone() as u64;
    let arg6 = self.buf[self.offs+6].clone() as u64;
    let arg7 = self.buf[self.offs+7].clone() as u64;
    let arg0 = arg0 << 56;
    let arg1 = arg1 << 48;
    let arg2 = arg2 << 40;
    let arg3 = arg3 << 32;
    let arg4 = arg4 << 24;
    let arg5 = arg5 << 16;
    let arg6 = arg6 << 8;
    self.offs += 8;
    let ret = arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | arg6 | arg7;
    Some(ret as usize)
  }

  ///decode a u8 length (if possible)
  ///and return a slice that long
  ///that will start right after the 1 length byte
  pub fn u8_encoded_slice(&mut self) -> Option<&'a [u8]> {
    let len = try_ret!(self.read_u8());
    self.take(len)
  }

  ///decode a u16 length (if possible)
  ///and return a slice that long 
  ///that will start right after the 2 length bytes
  pub fn u16_encoded_slice(&mut self) -> Option<&'a [u8]> {
    let len = try_ret!(self.read_u16());
    self.take(len)
  }

  ///decode a u24 length (if possible)
  ///and return a slice that long
  ///that will start right after the 3 length bytes
  pub fn u24_encoded_slice(&mut self) -> Option<&'a [u8]> {
    let len = try_ret!(self.read_u24());
    self.take(len)
  }

  ///decode a u32 length (if possible)
  ///and return a slice that long
  ///that will start right after the 4 length bytes
  pub fn u32_encoded_slice(&mut self) -> Option<&'a [u8]> {
    let len = try_ret!(self.read_u32());
    self.take(len)
  }

  ///decode a u64 length (if possible)
  ///and return a slice that long
  ///that will start right after the 8 length bytes
  pub fn u64_encoded_slice(&mut self) -> Option<&'a [u8]> {
    let len = try_ret!(self.read_u64());
    self.take(len)
  }

  ///return the remaining data in buffer as a PayLoad type
  pub fn payload(&mut self) -> Option<Payload<'a>> {
    Some(Payload(Cow::Borrowed(self.rest())))
  }
  ///decode a u8 length (if that is possible)
  ///and return a PayloadU8 type that contains
  ///a slice of that length
  pub fn u8_payload(&mut self) -> Option<PayloadU8<'a>> {
    let slice = try_ret!(self.u8_encoded_slice());
    Some(PayloadU8(Cow::Borrowed(slice)))
  }
  ///decode a u16 length (if that is possible)
  ///and return a PayloadU16 type that contains
  ///a slice of that length
  pub fn u16_payload(&mut self) -> Option<PayloadU16<'a>> {
    let slice = try_ret!(self.u16_encoded_slice());
    Some(PayloadU16(Cow::Borrowed(slice)))
  }
  ///decode a u24 length (if that is possible)
  ///and return a PayloadU24 type that contains
  ///a slice of that length
  pub fn u24_payload(&mut self) -> Option<PayloadU24<'a>> {
    let slice = try_ret!(self.u24_encoded_slice());
    Some(PayloadU24(Cow::Borrowed(slice)))
  }
  ///decode a u32 length (if that is possible)
  ///and return a PayloadU32 type that contains
  ///a slice of that length
  pub fn u32_payload(&mut self) -> Option<PayloadU32<'a>> {
    let slice = try_ret!(self.u32_encoded_slice());
    Some(PayloadU32(Cow::Borrowed(slice)))
  }
  ///decode a u64 length (if that is possible)
  ///and return a PayloadU64 type that contains
  ///a slice of that length
  pub fn u64_payload(&mut self) -> Option<PayloadU64<'a>> {
    let slice = try_ret!(self.u64_encoded_slice());
    Some(PayloadU64(Cow::Borrowed(slice)))
  }
}

/// Things we can encode and read from a Reader.
pub trait Codec: Debug + Sized {

  /// Encode yourself by appending onto `bytes`.
  fn encode(&self, bytes: &mut Vec<u8>);
  
  /// Read one of these from the front of `bytes` and
  /// return it.
  fn read<'a>(r: &mut Reader<'a>) -> Option<Self>;

  /// Convenience function to get the results of `encode()`.
  fn get_encoding(&self) -> Vec<u8> {
    let mut ret = Vec::new();
    self.encode(&mut ret);
    ret
  }
}


/* Encoding functions. */
pub fn encode_u8(v: u8, bytes: &mut Vec<u8>) {
  bytes.push(v);
}

pub fn decode_u8(bytes: &[u8]) -> Option<u8> {
  Some(bytes[0])
}


pub fn encode_u16(v: u16, bytes: &mut Vec<u8>) {
  bytes.push((v >> 8) as u8);
  bytes.push(v as u8);
}

pub fn decode_u16(bytes: &[u8]) -> Option<u16> {
  Some(((bytes[0] as u16) << 8) | bytes[1] as u16)
}


pub fn encode_u24(v: u32, bytes: &mut Vec<u8>) {
  bytes.push((v >> 16) as u8);
  bytes.push((v >> 8) as u8);
  bytes.push(v as u8);
}

pub fn decode_u24(bytes: &[u8]) -> Option<u32> {
  Some(((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | bytes[2] as u32)
}


pub fn encode_u32(v: u32, bytes: &mut Vec<u8>) {
  bytes.push((v >> 24) as u8);
  bytes.push((v >> 16) as u8);
  bytes.push((v >> 8) as u8);
  bytes.push(v as u8);
}

pub fn decode_u32(bytes: &[u8]) -> Option<u32> {
  Some(
       ((bytes[0] as u32) << 24) |
       ((bytes[1] as u32) << 16) |
       ((bytes[2] as u32) << 8) |
       bytes[3] as u32
      )
}


pub fn encode_u64(v: u64, bytes: &mut Vec<u8>) {
  let mut b64 = [0u8; 8];
  put_u64(v, &mut b64);
  bytes.extend_from_slice(&b64);
}

pub fn put_u64(v: u64, bytes: &mut [u8]) {
  bytes[0] = (v >> 56) as u8;
  bytes[1] = (v >> 48) as u8;
  bytes[2] = (v >> 40) as u8;
  bytes[3] = (v >> 32) as u8;
  bytes[4] = (v >> 24) as u8;
  bytes[5] = (v >> 16) as u8;
  bytes[6] = (v >> 8) as u8;
  bytes[7] = v as u8;
}

pub fn decode_u64(bytes: &[u8]) -> Option<u64> {
  Some(
       ((bytes[0] as u64) << 56) |
       ((bytes[1] as u64) << 48) |
       ((bytes[2] as u64) << 40) |
       ((bytes[3] as u64) << 32) |
       ((bytes[4] as u64) << 24) |
       ((bytes[5] as u64) << 16) |
       ((bytes[6] as u64) << 8) |
       bytes[7] as u64
      )
}

/*
pub fn encode_vec_u8<'a,T>(bytes: &'a mut Vec<u8>, items: &'a [T])
where T: Codec+'a
{
  let mut sub: Vec<u8> = Vec::new();
  for i in items {
    i.encode(&mut sub);
  }

  debug_assert!(sub.len() <= 0xff);
  encode_u8(sub.len() as u8, bytes);
  bytes.append(&mut sub);
}

pub fn encode_vec_u16<'a,T>(bytes: &'a mut Vec<u8>, items: &'a [T])
where T: Codec+'a
{
  let mut sub: Vec<u8> = Vec::new();
  for i in items {
    i.encode(&mut sub);
  }

  debug_assert!(sub.len() <= 0xffff);
  encode_u16(sub.len() as u16, bytes);
  bytes.append(&mut sub);
}

pub fn encode_vec_u24<'a,T>(bytes: &'a mut Vec<u8>, items: &'a [T])
where T: Codec+'a
{
  let mut sub: Vec<u8> = Vec::new();
  for i in items {
    i.encode(&mut sub);
  }

  debug_assert!(sub.len() <= 0xffffff);
  encode_u24(sub.len() as u32, bytes);
  bytes.append(&mut sub);
}

pub fn read_vec_u8<'a,T>(r: &'a mut Reader)
-> Option<Vec<T>>
where T: Codec+'a
{
  let mut ret: Vec<T> = Vec::new();
  let len = try_ret!(r.read_u8());
  let mut sub = try_ret!(r.sub(len));

  while sub.any_left() {
    ret.push(try_ret!(T::read(&mut sub)));
  }

  Some(ret)
}

pub fn read_vec_u16<'a,T>(r: &'a mut Reader)
-> Option<Vec<T>>
where T: Codec+'a
{
  let mut ret: Vec<T> = Vec::new();
  let len = try_ret!(r.read_u16());
  let mut sub = try_ret!(r.sub(len));

  while sub.any_left() {
    ret.push(try_ret!(T::read(&mut sub)));
  }

  Some(ret)
}

pub fn read_vec_u24<'a,T>(r: &'a mut Reader)
-> Option<Vec<T>>
where T: Codec+'a
{
  let mut ret: Vec<T> = Vec::new();
  let len = try_ret!(r.read_u24());
  let mut subslice = try_ret!(r.sub(len));
  None
}
*/
