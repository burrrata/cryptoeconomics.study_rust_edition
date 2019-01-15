// GOAL
//
// Refactor the code so that you can change any part
// and the "blockchain" still runs
//
// For example: 
// - the code uses a function called hash()
// - you can change the internal logic of the hash function, 
//   without chaning anything else and it all still works 
//   because hash() just takes in data and returns a string

// STF
//  - determines what is a valid state transition by verifying tx
//  - determines who is authorized to create a state change via PoA, PoW, PoS, etc...
//  - impliments the state change

// Data Encoding
//  - takes in arbitrary data and encodes it in a specific way
//  - the entire "blockchain" uses this in order to allow any function
//    to process arbitrary data inputs as well as sharing data between functions

// Hash
//  - takes in arbitrary data and returns a string
//  - the way that data is hashes or the encoding of the string can be changed

// Keys
//  - the method to generate public and private key pairs
//  - can be a centralized system, RSA, elliptic curves, etc...

// Accounts
//  - these will ALWAYS be a key/value pair in a HashMap
//  - what you can change is the data that the account struct holds
//  - UTXOs TBD



