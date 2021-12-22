# Blockchain Example with Rust
- Triển khai lại ý tưởng của (Learn Blockchains by Building One)[https://medium.com/@vanflymen/learn-blockchains-by-building-one-117428612f46]

# Cài đặt:
- Step1: Cargo build
- Step2: Cargo run (Mặc định với cổng 5000) HOẶC caro run -- --p 5001

# Các thư viện cài đặt:
    actix-web = "1.0"                                     // khởi tạo curd, api
    chrono = { version = "0.4.6", features = ["serde"]}  // Thời gian, utc, ..,
    serde = { version = "1.0.90", features = ["derive"] }
    serde_json = "1.0"
    crypto-hash = "0.3.3"                          // thuật toán băm
    uuid = { version = "0.7", features = ["v4"] } // gen uuid
    urlparse = "0.7.3"
    reqwest = "=0.9.17"                            // để call api                        

# Cách sử dụng
- Mine a new block: 

        curl http://localhost:5000/mine

- Creating a new transaction:

        curl -H "Content-Type: application/json" --request POST --data '{"sender":"1kyH6C7sivf9Q6r1ahZqKDywH1jkSY7uvCkhB1zwLVYgtsg",
        "recipient":"12imiRFgMGpPPVRiXLhQixuk1jMeTrQbzSJcZ4Bj7a3idmWT", 
        "amount":500}' http://localhost:5000/transactions/new

- View the full chain:

        curl http://localhost:5000/chain

- Add a new node:

        curl -H "Content-Type: application/json" --request POST --data '{"nodes":["http://localhost:5001"]}' http://localhost:5000/nodes/register

