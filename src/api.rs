use std::sync::Mutex;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::blockchain::{Block, Blockchain, Transaction};
use serde::{Deserialize, Serialize};

//               =============== Khoi tao cac struct ===============

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    nodes: Vec<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    total_nodes: Vec<String>,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    message: String,
    chain: Vec<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse  {
    message: String,
}

#[derive(Serialize)]
pub struct MiningRespose {
    message: String,
    index: u64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chain {
    pub chain: Vec<Block>,
    pub length: usize,
}

// =============== Cac ham api chinh ===================

pub fn new_transaction(
    state: web::Data<Mutex<Blockchain>>,
    req: web::Json<TransactionRequest>,
) -> HttpResponse {
    let sender = req.sender.to_owned();
    let recipient = req.recipient.to_owned();
    let index = state.lock().unwrap().new_transaction(&sender, &recipient, req.amount);

    HttpResponse::Created().json(MessageResponse {
        message: format!("Transaction will be add to block {}", index)
    })
}

pub fn mine(
    node_identifier: web::Data<String>,
    state: web::Data<Mutex<Blockchain>>,
    _req: HttpRequest,
)-> HttpResponse{
    let (proof, previous_hash) = {
        let blockchain = state.lock().unwrap();
        let last_block = blockchain.last_block().unwrap();
        let proof = Blockchain::proof_of_work(&last_block);
        let previous_hash = Blockchain::hash(last_block);
        (proof, previous_hash)
    };

    let mut blockchain = state.lock().unwrap();
    blockchain.new_transaction("0", &*&node_identifier, 1);
    let block = blockchain.new_block(proof, Some(&previous_hash));
    
    HttpResponse::Ok().json( MiningRespose {
        message: "Đã đào được 1 block".to_string(),
        index: block.index,
        transactions: block.transactions,
        proof,
        previous_hash,
    })

}


pub fn chain(
    state: web::Data<Mutex<Blockchain>>,
    _req: HttpRequest
)->HttpResponse {
    let length = state.lock().unwrap().chain.len();
    HttpResponse::Ok().json(Chain{
        chain: state.lock().unwrap().chain.clone(),
        length,
    })
}

pub fn register_node(
    state: web::Data<Mutex<Blockchain>>,
    req: web::Json<RegisterRequest>,
)-> HttpResponse {

    if req.nodes.is_empty(){
        return  HttpResponse::BadRequest().json(MessageResponse{
            message: "Error: Vui lòng cung cấp danh sách nodes hợp lệ".to_string(),
        });
    }

    let mut blockchain = state.lock().unwrap();
    for node in req.nodes.iter(){
        blockchain.register_node(node)
    }

    HttpResponse::Created().json(RegisterResponse{
        message: "Nodes mới đã được thêm".to_string(),
        total_nodes: blockchain.nodes.iter().cloned().collect(),
    })

}

pub fn resolve_nodes(
    state: web::Data<Mutex<Blockchain>>,
    _req: HttpRequest
)-> HttpResponse {
    let mut blockchain = state.lock().unwrap();
    let replaced = blockchain.resolve_conflicts();
    let message = if replaced {
        "Chuỗi đã được làm mới"
    } else {
        "Chuỗi không thay đổi"
    };

    HttpResponse::Ok().json(ResolveResponse{
        message: message.to_string(),
        chain: blockchain.chain.clone(),
    })
}