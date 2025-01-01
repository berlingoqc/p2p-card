use std::collections::HashMap;

use block::calculate_hash_block;
use chrono::Utc;
use protocol::generated::chain::{Block, BlockProposal, BlockProposalFinalResponse, BlockProposalResponse, Blockchain, ProposalResponseType, Transaction};

use super::{deck::encryption::generate_random_nonce, encryption::SealingKey};

use base64::Engine;

pub mod block;


pub struct BlockChainManager {

    sealing_key: SealingKey,

    my_pub_key: String,

    current_chain: Blockchain,

    current_proposal: HashMap<String, BlockProposal>,

    current_approuval: HashMap<String, HashMap<String, BlockProposalResponse>>,

    number_people: u32,

    threshold: u32,

}

impl BlockChainManager {

    pub fn create(chain: Blockchain, sealing_key: SealingKey, number_people: u32, threshold: u32) -> Self {

        let my_pub_key: String = base64::engine::general_purpose::STANDARD.encode(&sealing_key.get_pub_key().unwrap());
        Self { 
            sealing_key: sealing_key,
            my_pub_key: my_pub_key,
            current_chain: chain,
            current_approuval: HashMap::new(),
            current_proposal: HashMap::new(),
            number_people: number_people,
            threshold: threshold,
        }
    }

    pub fn create_transaction(&self, action: u32, content: &Vec<u8>) -> Result<Transaction, ()> {

        let mut t = Transaction::default();

        t.pub_key = self.my_pub_key.clone();
        t.action = action;

        t.signature = self.sealing_key.encrypt(&generate_random_nonce(), content).unwrap();
        t.timestamp = self.get_current_timestamp_utc();

        Ok(t)

    }

    pub fn create_block(&mut self, transactions: Vec<Transaction>) -> Result<BlockProposal, ()> {

        let mut block = Block::default();
        block.previous_hash = self.get_last_block().map(|b| calculate_hash_block(&block).unwrap()).or_else(|| Some("0".to_string()))
            .unwrap();

        block.timestamp = self.get_current_timestamp_utc();
        block.transactions = transactions;

        let mut block_proposal = BlockProposal::default();
        block_proposal.block = Some(block.clone());
        block_proposal.pub_key = self.my_pub_key.clone();
        block_proposal.block_hash = calculate_hash_block(&block).unwrap();

        self.current_proposal.insert(block_proposal.block_hash.clone(), block_proposal.clone());

        Ok(block_proposal)
    }


    pub fn on_block_proposal(&mut self, proposal: &BlockProposal) -> Result<(), ()> {
        if let Some(previous_proposal) = self.current_proposal.insert(proposal.pub_key.clone(), proposal.clone()) {
            println!("dropping previous proposal for {}", previous_proposal.pub_key);
        }
        Ok(())
    }

    pub fn on_block_proposal_response(&mut self, proposal_response: &BlockProposalResponse) -> Result<(), ()> {
        let mut current_responses = if let Some(v) = self.current_approuval.get_mut(&proposal_response.block_hash) {
            v
        } else {
            let map = HashMap::new();
            self.current_approuval.insert(proposal_response.block_hash.clone(), map);
            self.current_approuval.get_mut(&proposal_response.block_hash).unwrap()
        };

        // need to validate integrity of this shit myself
        // * is hash of block valid
        // * are all signature valid and some as owner
        if let Some(previous_response) = current_responses.insert(proposal_response.pub_key.clone(), proposal_response.clone()) {
            println!("dropping previous proposal response for {}", previous_response.pub_key);
        }

        Ok(())
    }

    pub fn get_response_block_proposal(&mut self, block_hash: &String, response: ProposalResponseType) -> Result<BlockProposalResponse, ()> {
        let mut proposal_response = BlockProposalResponse::default();
        proposal_response.block_hash = block_hash.clone();
        proposal_response.pub_key = self.my_pub_key.clone();
        proposal_response.response = response as i32;

        self.on_block_proposal_response(&proposal_response).unwrap();

        Ok(proposal_response)
    }

    pub fn get_final_response_block_proposal(&mut self, block_hash: &String) -> Result<BlockProposalFinalResponse, ()> {

        let proposal = self.current_proposal.get(block_hash).unwrap();
        let responses = self.current_approuval.get(block_hash).unwrap();
        let len = responses.len() as u32;

        if len <= self.number_people {
            eprintln!("not enought response to produce final response");
            return Err(());
        }

        let mut nbr_accepted = 0;

        for (_, v) in responses.iter() {
            match ProposalResponseType::from_i32(v.response).unwrap() {
                ProposalResponseType::Accepted => nbr_accepted += 1,
                _ => {},
            }
        }

        let response = if nbr_accepted >= self.threshold {
            ProposalResponseType::Accepted
        } else {
            ProposalResponseType::Refused
        };

        if response == ProposalResponseType::Accepted {
            let block = proposal.block.as_ref().unwrap();
            self.current_chain.chain.insert(self.current_chain.chain.len(), block.clone());
        }

        Ok(BlockProposalFinalResponse { response: response as i32, block_hash: proposal.block_hash.clone(), chain_hash: "".to_string(), chain_length: self.current_chain.chain.len() as u32 })
    }

    fn get_last_block(&self) -> Option<&Block> {
        let len = self.current_chain.chain.len();
        if len == 0 {
            return None;
        }
        self.current_chain.chain.get(len - 1)
    }

    fn get_current_timestamp_utc(&self) -> u64 {
        Utc::now().timestamp() as u64
    }

}



#[cfg(test)]
mod tests {
    use protocol::generated::chain::{Blockchain, ProposalResponseType};

    use crate::logic::{encryption::SealingKey, players::key_loader::{KeyLoader, RandomKeyLoader}};

    use super::BlockChainManager;


    #[test]
    pub fn test_block_proposal() {

        let alice_keypair = RandomKeyLoader{}.load_key_pair().unwrap();
        let john_keypair = RandomKeyLoader{}.load_key_pair().unwrap();

        let alice_sealing_key = SealingKey::create(&alice_keypair.1, alice_keypair.0.clone());
        let john_sealing_key = SealingKey::create(&john_keypair.1, john_keypair.0.clone());

        // Blockchain is created by the first player , or they start by a agreed chain
        let chain = Blockchain::default();

        let mut alice_chain_manager = BlockChainManager::create(chain.clone(), alice_sealing_key, 2, 2);
        let mut john_chain_manager = BlockChainManager::create(chain.clone(), john_sealing_key, 2, 2);


        // Alice create a new block proposal , accepted it and send the proposal the john
        let t1 = alice_chain_manager.create_transaction(15, &b"berlingoqc p2p-card".to_vec()).unwrap();
        let alice_block_proposal = alice_chain_manager.create_block(vec![t1]).unwrap();
        let alice_block_proposal_response = alice_chain_manager.get_response_block_proposal(&alice_block_proposal.block_hash, ProposalResponseType::Accepted).unwrap();


        john_chain_manager.on_block_proposal(&alice_block_proposal).unwrap();
        let john_proposal_respone = john_chain_manager.get_response_block_proposal(&alice_block_proposal.block_hash, ProposalResponseType::Accepted).unwrap();

        // Each other process the other response
        john_chain_manager.on_block_proposal_response(&alice_block_proposal_response).unwrap();
        alice_chain_manager.on_block_proposal_response(&john_proposal_respone).unwrap();


        // They should next call to process the final response and edit the blockchain
        let final_alice = alice_chain_manager.get_final_response_block_proposal(&alice_block_proposal.block_hash).unwrap();
        let final_john = john_chain_manager.get_final_response_block_proposal(&alice_block_proposal.block_hash).unwrap();

        assert_eq!(final_alice.block_hash, final_john.block_hash);
        assert_eq!(final_alice.chain_length, final_john.chain_length);

    }


}