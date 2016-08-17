#![crate_type = "staticlib"]

extern crate multihash;
extern crate bytes;

use multihash::{encode, decode};
use multihash::HashTypes::SHA2256 as Hash256;

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {

    let hash = encode(Hash256, "my hash").unwrap();
    let multi = decode(&hash).unwrap();
    println!("{:?}", multi);
    input * 2
}

#[no_mangle]
pub extern fn sha256(data: &[u8], length: u32) -> i32 {
    let sss = bytes::Bytes::from_slice(data);
    let result = std::str::from_utf8(data);
    let str = match result {
        Err(e) => {
            println!("Got error {:?}", e);
            "error"
        }
        Ok(x) => {
            println!("Got {:?}", x);
            x
        }
    };
    println!("String is {:?}", str);
    let hash = encode(Hash256, str).unwrap();
    let multi = decode(&hash).unwrap();
    println!("The mulithash: {:?}", multi);
    println!("Digest: {:?}", multi.digest);
    3
}

#[test]
fn it_works() {
    println!("Printing something");
    let str = "string";
    let array:&[u8] = b"123 456";
    let sss = bytes::Bytes::from_slice(array);
    // let data = std::str::to_utf8(str);
    // let data: [u8; 4] = [1,2,3,4];
    sha256(&array);
}
