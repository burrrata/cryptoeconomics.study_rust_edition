pub struct DataEncoding;

impl DataEncoding {
    
    // TODO
    //
    // - Upgrade to something like what Substrate uses
    //   https://github.com/paritytech/substrate/tree/master/core/serializer
    // - Also, does it need it's own struct/impl or does it
    //   make sense to have it in the State impl?
    
    // Turn stuff into an &[u8] slice
    pub unsafe fn to_u8<T: Sized>(p: &T) -> &[u8] {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }    

    // i32 -> String
    // https://doc.rust-lang.org/nightly/std/string/trait.ToString.html
    pub fn i2s(input: i32) -> String {
        
        let output = input.to_string();
        
        output
    }
    
    // String -> i32
    // https://stackoverflow.com/questions/27043268/convert-a-string-to-int-in-rust
    pub fn s2i(input: String) -> i32 {
        
        let output = input.parse::<i32>().unwrap();
        
        output
    }

    // string -> Vec<i32>
    pub fn s2v(input: String) -> Vec<i32> {
        
        let output: Vec<i32> = input.as_bytes()
                                    .iter()
                                    .map(|x| *x as i32)
                                    .collect();
        
        output
    }
 
    // Vec<i32> -> String
    // https://doc.rust-lang.org/nightly/std/string/trait.ToString.html
    pub fn v2s(input: Vec<i32>) -> String {
        
        let mut output_vec = Vec::new();
        for i in input {
            output_vec.push(i.to_string())
        }
        let output_string = output_vec.join("");
        
        output_string
    }
}

