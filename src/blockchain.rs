use chrono::{DateTime, Utc};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crypto_hash::{hex_digest, Algorithm};
use urlparse::urlparse;
use reqwest;

use crate::api::Chain;


#[derive(Clone, Hash, Serialize, Deserialize, Debug)]
pub struct Transaction {
    sender: String,   // Dia chi nguoi gui
    recipient: String, // Dia chi nguoi nhan
    amount: i64, // So luong
}

#[derive(Clone, Hash, Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

#[derive(Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    pub nodes: HashSet<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain{
        let mut blockchain = Blockchain{
            chain: vec![],
            current_transaction: vec![],
            nodes: HashSet::new(),
        };

        blockchain.new_block(100, Some("0"));
        blockchain
    }
    /// Tao mot block trong blockchain
    /// 
    /// :param proof: duoc tao boi thuat toan POW (Proof of Work)
    /// :param previos_hash: (Optional) hash cua block truoc do
    /// :return: New Block

    pub fn new_block(&mut self, proof:u64, previous_hash: Option<&str>)-> Block {
        let block = Block {
            index: (self.chain.len() +1 ) as u64,
            timestamp: Utc::now(),
            transactions: self.current_transaction.drain(0..).collect(),
            proof,
            previous_hash: previous_hash.unwrap_or("1").to_string(),

        };

        self.chain.push(block.clone());
        block
    }

    /// Tạo một transaction 
    /// 
    /// :param sender: Địa chỉ người gửi
    /// :param recipient: Địa chỉ người nhận
    /// :param amount: Amount
    /// :return index của block sẽ chứa transaction này.
    
    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: i64)-> u64 {
        self.current_transaction.push(Transaction{
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
        });

        self.last_block().unwrap().index + 1
    }
    
    pub fn last_block(&self)-> Option<&Block>{
        self.chain.last()
    }

    
    /// Thuật toán Proof of work đơn giản
    /// Tìm một số k, sao cho hash(k) kết thúc bằng "0000"
    pub fn proof_of_work(last_block:&Block) -> u64{
        let mut proof = 0;
        let last_proof = last_block.proof;
        let last_hash = &last_block.previous_hash;
        while !Self::valid_proof(last_proof, proof, last_hash) {
            proof +=1;
        }
        proof
    }

    /// Thuật toán kiểm tra mã hash hợp lệ, ở đây sử dụng SHA256
    pub fn valid_proof(last_proof: u64, proof:u64, last_hash:&String)->bool {
        let guess = format!("{}{}{}", last_proof, proof, last_hash);
        let guess_hash = hex_digest(Algorithm::SHA256, guess.as_bytes());
        guess_hash.ends_with("0000")
    }

    pub fn hash(block: &Block) ->String{
        let serialized = serde_json::to_string(&block).unwrap();
        hex_digest(Algorithm::SHA256, serialized.as_bytes())

    }

    /// Kiểm tra chuỗi có hơp lệ hay không
    pub fn valid_chain(&self, chain: &[Block])->bool{
        let mut last_block =&chain[0];
        let mut current_index = 1;
        
        while current_index < chain.len() {
            let block = &chain[current_index];
            println!("{:?}", last_block);
            println!("{:?}", block);
            println!("---------------");

            if block.previous_hash != Blockchain::hash(last_block){
                return false;
            }

            if !Blockchain::valid_proof(last_block.proof, block.proof, &last_block.previous_hash){
                return false
            }

            last_block = block;
            current_index +=1;
        }

        true
    }

    /// Thêm một node mới vào danh sách (1 endpoint)
    /// :param address của node ví dụ như: http://10.1.46.32:3400
    pub fn register_node(&mut self, address: &str){
        let parsed_url = urlparse(address);
        self.nodes.insert(parsed_url.netloc);
    }

    /// Giải quyết xung đột bằng cách lấy chuỗi dài nhất có trong mạng
    pub fn resolve_conflicts(&mut self) -> bool {
        let mut max_length = self.chain.len();
        let mut new_chain: Option<Vec<Block>> = None;

        // Xác minh các chuỗi trong Network

        for node in &self.nodes {
            let mut response = reqwest::get(&format!("http://{}/chain", node)).unwrap();
            
            if response.status().is_success() {
                let node_chain: Chain  = response.json().unwrap();
                if node_chain.length > max_length && self.valid_chain(&node_chain.chain){
                    max_length = node_chain.length;
                    new_chain = Some(node_chain.chain)
                }
            }

        }
        
        // Thay the chuoi hien tai bằng chuỗi dài hơn, hợp lệ được tìm thấy.
        match new_chain {
            Some(x) => {
                self.chain = x;
                true
            }
            None => false
            
        }
    }
}