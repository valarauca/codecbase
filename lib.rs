
#[macro_use]
mod macros;
mod codec;
mod base;

pub use codec::{Codec,Reader};
pub use base::{Payload,PayloadU24,PayloadU16,PayloadU8};
pub use codec::{
    encode_u8,
    decode_u8,
    read_u8,
    encode_u16,
    decode_u16,
    read_u16,
    encode_u24,
    decode_u24,
    read_u24,
    encode_u32,
    decode_u32,
    read_u32
};


//
//Below here are tests to validate _how_ Reader, Codec, and Payload work
//


//Reader
#[test]
fn validate_reader() {
    
    //According to my understand reader is a borrowed slice + a len
    //When a read happens the `offs` value is modified, incremented
    //to account for the new data being _removed/read_ from the
    //structure
    //
    //The goal of this test is to help myself understand it's behavior
    //as well as extensively validate it's functionality.
    //
    //The { } blocks are used to ensure the borrow checker does not
    //get angry with me
    
    
    //construct test data
    let test_data: [u8;10] = [0,1,2,3,4,5,6,7,8,9];
    let mut r = Reader::init( &test_data);

    //validate initial state of the buffer
    {
        assert_eq!( r.rest(), &test_data);
        assert_eq!( r.any_left(), true);
        assert_eq!( r.left(), 10);
        assert_eq!( r.used(), 0);
    }

    //take will copy data out of the buffer
    {
        let x = r.take(2).unwrap();
        //validate the new data
        assert_eq!( x, &[0u8,1u8]);
    }{
        //see what is left in read
        assert_eq!( r.rest(), &[2,3,4,5,6,7,8,9]);
        assert_eq!( r.any_left(), true);
        assert_eq!( r.left(), 8);
        assert_eq!( r.used(), 2);
    }

    //Sub borrowed reader, of the remaining data
    {
        //you cannot borrow more data then there is left
        let x = r.sub(10);
        assert!(x.is_none());
    }{
        //create a new reader of all the data left in r
        let x = r.sub(8).unwrap();
        assert_eq!( x.rest(), &[2,3,4,5,6,7,8,9]);
        assert_eq!( x.any_left(), true);
        assert_eq!( x.left(), 8);
        //this operation _resets_ the used count
        assert_eq!( x.used(), 0);
    }

    //validate the state of the empty reader
    {
        assert_eq!( r.any_left(), false);
        assert_eq!( r.used(), 10);
        assert_eq!( r.left(), 0);
    }
}

//Encode/Decode
#[test]
fn validate_encode_decode_read() {
    
    //Encode, Decode, and Read functions are for insert, and removing
    //integers from values
    
    
    //build test data
    let mut v: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9];
    
    //validate date data
    assert_eq!(v.len(), 10);
    assert_eq!(v.as_slice(), &[0,1,2,3,4,5,6,7,8,9]);
    
    //encode a u8 value
    encode_u8(80, &mut v);
    assert_eq!(v.len(), 11);
    assert_eq!(v.as_slice(), &[0,1,2,3,4,5,6,7,8,9,80]);
}

/*
//PayloadU8
#[test]
fn validate_payloadu8() {
    
    //A zero-sized type that wrapped Vec<u8> to
    //represent a u8 value is encoded on the start of it

    //build test data
    let x = PayloadU8::new(Vec::new());
    assert_eq!( x.len(), 0);

    //create data
    let test_data: [u8;11] = [10,0,1,2,3,4,5,6,7,8,9];
    let mut r = Reader::init( &test_data);
    
    //build a payload
    let payload = PayloadU8::read(&mut r).unwrap();
    assert_eq!(payload.len(), 10);
    assert_eq!(payload.0.as_slice(), &[0,1,2,3,4,5,6,7,8,9]);
}
*/
