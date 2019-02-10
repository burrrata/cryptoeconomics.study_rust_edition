// This struct holds all the data needed for 
// the chosen state transition protocol.
// In this case we're doign PoS, but if you
// wanted to impliment PoW you would write a new
// STF struct and new verify_pending_tx and proof
// functions.
#[derive(Debug, Clone)]
pub struct STF {
    version: String, // PoA, PoW, PoS, etc...
    difficulty: i32,
    validator: i32,
}

impl STF {
    
    // Standin for a "random beacon"
    pub fn random_validator_selection(state: &mut State) {
        
        // kick out any validators who don't meet the 
        // difficulty requirements
        // TODO: find a more "rust like" way to do this
        // without clone()
        let state_copy = state.clone();
        let mut count = 0;
        for i in state_copy.validators {
            if state.accounts.get(&i).unwrap().balance < state.stf.difficulty {
                state.validators.remove(count as usize);
            }
            count += 1;
        }
        
        // check that there are validators
        if state.validators.len() <= 0 {
            println!("ERROR: no known validators.");
            return
        }
        
        // randomly select a validator from the validator Vec
        let validator_num = thread_rng().gen_range(0, state.validators.len());
        state.stf.validator = state.validators[validator_num];
    }
    
    // This function encodes the rules of what qualifies as a "valid tx"
    pub fn verify_pending_tx(state: &mut State) -> Vec<TX> {
        
        let mut verified_tx = Vec::new();
        
        for i in &state.pending_tx {
        
            if !(state.accounts.contains_key(&i.data.sender)) {
                println!("Invalid TX: sender not found.");
                continue
            }
            
            if !(state.accounts.contains_key(&i.data.receiver)) {
                println!("Invalid TX: receiver not found.");
                continue
            }
            
            if !(i.data.amount > 0) {
                println!("Invalid TX: negative amount error.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                continue
            }
            
            if !(state.accounts.get(&i.data.sender).unwrap().balance > i.data.amount) {
                println!("Invalid TX: insufficient funds.");
                println!("{} cannot send {} to {}", i.data.sender, i.data.amount, i.data.receiver);
                continue         
            }
            
            if !(i.data.sender_nonce == state.accounts.get(&i.data.sender).unwrap().nonce) {
                println!("Invalid TX: potential replay tx.");
                println!("{} has nonce {}, but submitted a tx with nonce {}", i.data.sender, state.accounts.get(&i.data.sender).unwrap().nonce, i.data.sender_nonce);
                continue
            }
            
            if !(Keys::check_tx_signature(state.keys, i.clone())) {
                println!("Invalid TX: signature check failed");
                continue
            }
            
            verified_tx.push(i.clone());
        }
        
        verified_tx
    }

    // This function creates a proof that authorizes the state transition
    // This can be as complex as desired such as in a PoW setting 
    // Or it can simply hash the publicly announced validator address like
    // it does here :)
    pub fn proof(state: &State) -> String {
    
        let hash = Hash::hash(&state.stf.validator);
        
        hash
    }
    
    // Create A New Block With Valid Transactions
    pub fn create_block(state: &mut State) -> Block {
    
        let verified_tx = STF::verify_pending_tx(state);
        let header = BlockHeader {
            nonce: 0,
            timestamp: time::now().to_timespec().sec as i32,
            block_number: state.history.last().unwrap().data.header.block_number + 1,
            previous_block_hash: Hash::hash(&state.history.last().unwrap().data.header.current_block_hash),
            current_block_hash: Hash::hash_tree(verified_tx.clone()),
        };
        
        let data = BlockData {
            header: header,
            transactions: verified_tx, 
        };
        let proof = STF::proof(state);
        
        let block = Block {
            proof: proof,
            data: data,
        };
        
        block
    }
    
    // function to transition the state
    pub fn check_block(state: &State,
                       block: &Block) -> bool {
        
        // proof to check
        let submitted_proof = &block.proof;
        
        // check that validator matches randomly chosen STF validator
        if submitted_proof != &Hash::hash(&state.stf.validator) {
            println!("\nProof Error: invalid PoS validator.");
            return false
        }
        
        // check validator account has enough state
        let validator_balance = state.accounts.get(&state.stf.validator).unwrap().balance;
        if !(validator_balance > state.stf.difficulty) {
            println!("ERROR: block proof does not meet difficulty requirements.");
            return false
        }
        
        // if tests are passed, return true
        true
    }
    
}

