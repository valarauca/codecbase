
#[macro_use]
mod macros;
mod codec;

pub use codec::{Codec,Reader};
//pub use base::{Payload,PayloadU24,PayloadU16,PayloadU8};


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

