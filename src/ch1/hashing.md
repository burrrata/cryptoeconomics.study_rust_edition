# Hashing
All about those trees

``
// TBD
``

# Resources
- https://en.wikipedia.org/wiki/Merkle_tree
- https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/
- https://ethereum.stackexchange.com/questions/2100/what-is-a-block-hash


# WIP STUFF
- uses unsafe code, but no external libraries 


```rust
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;


struct MyStruct {
    id: u8,
    data: String,
}

#[derive(Debug, Clone)]
struct TX {
    sender: String,
    receiver: String,
    tx_amount: f32,
    nonce: i32,
}

#[derive(Debug, Clone, Hash)]
struct AltTX {
    sender: String,
    receiver: String,
    tx_amount: i32,
    nonce: i32,
}


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

fn hash(stuff: &[u8]) -> String {
    
    let mut hasher = DefaultHasher::new();
    hasher.write(stuff);
    let digest = hasher.finish();
    let hex_digest = format!("{:#X}", digest);
        
    hex_digest
}


fn main() {

    // Using Example Struct
    let my_struct = MyStruct {
        id: 98,
        data: "Hello World".to_string(),
    };
    
    let bytes: &[u8] = unsafe { 
        any_as_u8_slice(&my_struct)
    };

    println!("{:?}", &bytes);
    
    
    // Using TX Struct
    let tx = TX {
        sender: "Your Mom".to_string(),
        receiver: "Yours truly".to_string(),
        tx_amount: 1000.0,
        nonce: 345,
    };
    
    let tx_bytes: &[u8] = unsafe {
        any_as_u8_slice(&tx)
    };
    println!("tx: {:?}", &tx_bytes);
    
    let tx_hash = hash(tx_bytes);
    println!("tx hash: {:#?}", tx_hash);
}




```
