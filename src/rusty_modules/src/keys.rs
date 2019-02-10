

// This struct holds all the data for the key generation
// and signing. If you want to use a different key
// protocol, change the data in the Keys struct as well
// as the functions in the Keys impl
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    min: i32,
    max: i32,
    p: i32,
    q: i32,
    modulo: i32,
    ctf_pq: i32, 
}

/// "RSA" Key Generation and Signing ///
impl Keys {
    
    // These functionsare not needed as we have hard coded
    // the modulo and ctf_pq values
    /*
    // greatest common divisor
    pub fn gcd(a: i32,
               b: i32) -> i32 {
        
        let (mut a, mut b) = if a > b {
            (a, b)
        } else {
            (b, a)
        };
    
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
    
        a
    }
    
    // lowest common multiple
    pub fn lcm(a: i32,
               b: i32) -> i32 {
        
        let lcm = (a * b) / Keys::gcd(a, b);
        
        lcm
    }
    
    // Carmichael's totient function
    pub fn ctf(a: i32,
               b: i32) -> i32 {
        
        Keys::lcm(a - 1, b - 1)
    }
    */
    
    // slowly check if a number is prime
    pub fn slow_prime_check(self,
                            num: i32) -> bool {
        
        if num < self.min {
            println!("number must be greater than {}", self.min);
        }
        
        if num > self.max {
            println!("number cannot be greater than {}", self.max);
        }
        
        for i in 2..num{
            if num % i == 0 {
                return false
            }
        }
        
        true
    }

    // slowly, yet randomly, generate a prime number within a range
    pub fn prime_gen(self) -> i32 {
        
        for _i in 0..self.max {
            let p = thread_rng().gen_range(self.min, self.max);
            if Keys::slow_prime_check(self, p) {
                return p
            }
        }
        
        0
    }

    // generate a private key within a range
    pub fn priv_key_gen(self) -> i32 {
        
        let priv_key = Keys::prime_gen(self);
        assert!(self.max % priv_key != 0);
        
        priv_key
    }
    
    // slowly find the modular multiplicative inverse of a prime 
    pub fn slow_mmi(self,
                    priv_key: i32)-> i32 {
        
        for i in 2..self.max {
            if (i * priv_key) % self.ctf_pq == 1 {
                return i
            }
        }
        println!("Try larger search?");
        
        0
    }
    
    // create a public key from a pricate key and RSA param data
    pub fn pub_key_gen(self,
                       priv_key: i32) -> i32 {
        
        let pub_key = Keys::slow_mmi(self, priv_key);
        
        pub_key
    }
    
    // generate a private/public key pair
    pub fn generate_keypair(self) -> (i32, i32){
        let priv_key = Keys::priv_key_gen(self);
        let pub_key = Keys::pub_key_gen(self, priv_key);
        (priv_key, pub_key)
    }
    
    // Because... Rust.
    pub fn exp_mod(self,
                   input: i32,
                   power: i32) -> i32 {
        
        let mut out = (input * input) % self.modulo;
        // because the first iter of out took 2 off the base
        for _i in 0..power-2 {
            out = (out * input) % self.modulo;
        }
        
        out
    }
    
    // Sign a TX with a toy RSA function
    pub fn sign<T>(self,
                   thing_to_be_signed: &T,
                   signing_key: i32) -> Vec<i32> {
        
        let hashed_thing = Hash::hash(thing_to_be_signed);
        
        let mut hashed_thing_vec = Vec::new();
        for i in hashed_thing.chars() {
            hashed_thing_vec.push(i.to_string().parse::<i32>().unwrap())
        }
        
        let mut signed_vec = Vec::new();
        for i in hashed_thing_vec {
            signed_vec.push(Keys::exp_mod(self, i, signing_key,));
        }

        signed_vec
    }
    
    // Check signature on a TX
    pub fn check_tx_signature(self,
                              tx: TX) -> bool {
        
        let tx_sig_check: Vec<i32> = tx.clone().signature;
        
        let mut tx_sig_check_pub_signed = Vec::new();
        for i in tx_sig_check {
            tx_sig_check_pub_signed.push(Keys::exp_mod(self, i, tx.data.sender))
        }
        
        let mut tx_sig_check_string = String::new();
        for i in tx_sig_check_pub_signed {
            tx_sig_check_string.push_str(&i.to_string())
        }
        
        let hashed_tx = Hash::hash(&tx.data);
        
        if tx_sig_check_string == hashed_tx {
            return true
        } else {
            return false
        }
    }
}

