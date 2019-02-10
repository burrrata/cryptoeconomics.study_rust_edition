#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Account {
    balance: i32,
    nonce: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxData {
    sender: i32,
    sender_nonce: i32,
    amount: i32,
    receiver: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TX {
    data: TxData,
    signature: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    nonce: i32,
    timestamp: i32,
    block_number: i32,
    previous_block_hash: String,  
    current_block_hash: String,  
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockData {
    header: BlockHeader,
    transactions: Vec<TX>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    proof: String,
    data: BlockData,
}

// TODO
// - does it make sense to add more data to the State?
//   STF (type, difficulty, etc...)
//   KEY_PARAMS (type, p, q, modulo, etc..)
//   or maybe CRYPTO (KEY_PARAMS, hash function, hash tree function, etc...)
#[derive(Debug, Clone)]
pub struct State {
    keys: Keys,
    stf: STF,
    accounts: HashMap<i32, Account>,
    validators: Vec<i32>,
    pending_tx: Vec<TX>,
    history: Vec<Block>,
}

impl State {

    // Create a new state
    pub fn create_state() -> State {
        
        let rsa_params = Keys {
            min: 0,
            max: 1000000,
            p: 61,
            q: 53,
            modulo: 3233,
            ctf_pq: 780,
        };

        let stf_data = STF {
            version: String::from("PoS"),
            difficulty: 100,
            validator: 0,
        };

        let genesis_block = Block {
                proof: String::from("GENESIS BLOCK"),
                data: BlockData {
                    header: BlockHeader {
                        nonce: 0,
                        timestamp: time::now().to_timespec().sec as i32,
                        block_number: 0,
                        previous_block_hash: String::from("N/A"),  
                        current_block_hash: Hash::hash(&String::from("")),  
                    },
                    transactions: Vec::new(),
                }
            };
        
        let new_state = State {
            keys: rsa_params,
            stf: stf_data,
            accounts: HashMap::new(),
            validators: Vec::new(),
            pending_tx: Vec::new(),
            history: vec![genesis_block],
        };
        
        new_state
    }

    // Create a new account
    pub fn create_account(&mut self) {
        
        // TODO
        // - How can I make Keys::generator_keypair() not
        //   take in anything as input and have all the params
        //   stored within the Keys library?
        let (priv_key, pub_key) = Keys::generate_keypair(self.keys);
        let new_account = Account {
            balance: 0,
            nonce: 0,
        };
        
        if self.accounts.contains_key(&pub_key) {
            println!("Bummer... account collision.");
            return
        }
        
        self.accounts.insert(pub_key, new_account);
        //println!("\nThis is your public key: {:#?}", &pub_key);
        //println!("This is your private key: {:#?}", &priv_key);
        //println!("This is your account: {:#?}", self.accounts.get(&pub_key).unwrap());
    }
    
    // Create a new TX
    pub fn create_tx(&mut self,
                     sender_pub_key: i32,
                     sender_priv_key: i32,
                     receiver_pub_key: i32,
                     amount: i32) {
        
        
        let data = TxData {
            sender: sender_pub_key,
            sender_nonce: self.accounts.get(&sender_pub_key).unwrap().nonce,
            receiver: receiver_pub_key,
            amount: amount,
        };
        
        let signature = Keys::sign(self.keys, &data, sender_priv_key);
        
        let tx = TX {
            data: data,
            signature: signature,
        };
        
        self.pending_tx.push(tx);
    }
    
    // function to add an account to the validator Vec
    pub fn create_validator(&mut self,
                                account: i32) {
        
        self.validators.push(account);
    }
    
    // function to transition the state to a new state
    pub fn create_new_state(&mut self) {
        
        // "publicly" select a random validator
        STF::random_validator_selection(self);
        
        // check tx and put valid ones into a block
        let mut block = STF::create_block(self);
        
        // check that the block proof is valid
        if !(STF::check_block(&self, &block)) {
            // if block is not valid slash validator's funds
            println!("\nERROR: block not valid.");
            self.accounts.get_mut(&self.stf.validator).unwrap().balance -= self.stf.difficulty;
            return
        }
        
        // if block is valid add reward to validator's balance
        self.accounts.get_mut(&self.stf.validator).unwrap().balance += self.stf.difficulty;
        
        // transition the state by incorporating the
        // information in the new block
        for i in &block.data.transactions {
            self.accounts.get_mut(&i.data.sender).unwrap().balance -= i.data.amount;
            self.accounts.get_mut(&i.data.receiver).unwrap().balance += i.data.amount;
            self.accounts.get_mut(&i.data.sender).unwrap().nonce += 1;
        }
        
        // add the block to the history
        self.history.push(block);
    }
}

