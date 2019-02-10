
pub struct Hash;

impl Hash {
    
    // Takes a preimage ("preimage" = fancy word for input to a hash function)
    // Encodes it via the data_encode() function
    // Hashes that data into a hex or an integer (you choose)
    fn hash<T>(preimage: &T) -> String {
        
        // convert to u8
        let stuff_as_u8 = unsafe {
            DataEncoding::to_u8(preimage)
        };
        
        // hash u8 to u64
        let mut hasher = DefaultHasher::new();
        hasher.write(stuff_as_u8);
        
        // format u64 hash as String
        let string_digest = format!("{}", hasher.finish());
        string_digest
        
        // hex String
        //let digest = hasher.finish();
        //let hex_digest = format!("{:#X}", digest);
        //hex_digest
        
        // i32
        //let digest = hasher.finish() as i32;
        //digest 
        
        // f64
        //let digest = hasher.finish() as f64;
        //digest 
     
        // u64
        //let digest = hasher.finish();
        //digest
    }   
    
    // Create A Merkle Tree Of All TX In A Vec
    pub fn hash_tree<T>(stuff: Vec<T>) -> String {
        
        let mut v = Vec::new();

        for i in &stuff {
            let hashed = Hash::hash(&i);
            v.push(hashed);
        }

        if v.len() % 2 == 1 {
            let last = v.last().cloned().unwrap();
            v.push(last);
        }

        while v.len() > 1 {
            let mut h1 = v.remove(0);
            let mut h2 = v.remove(0);
            h1.push_str(&mut h2);
            let nh = Hash::hash(&h1);
            v.push(nh);
        }
        
        v.pop().unwrap()
    }
    
}
