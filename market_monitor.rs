use anyhow::{Context, Result};
use alloy::{
    consensus::Transaction,
    network::{Ethereum, TransactionResponse},
    primitives::{Address, B256},
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
    sol_types::SolCall, // <--- 修正解码错误
    sol, // 引入 sol! 宏
   
};




use futures_util::StreamExt;
use std::sync::Arc;
use url::Url;

use alloy::primitives::Bytes;
use hex::FromHex; 

// 引用 boundless_market crate 中的合约接口
//use boundless_market::contracts::IBoundlessMarket;

// 设置 rustls 加密提供程序
use rustls::crypto::ring::default_provider;

const MARKET_ADDRESS: Address = alloy::primitives::address!("26759dbb201afba361bec78e097aa3942b0b4ab8");





sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    IBoundlessMarket,
    "abi/BoundlessMarket.json"
}


/// 处理单个交易哈希的辅助函数
async fn process_transaction<P: Provider<Ethereum>>(provider: Arc<P>, tx_hash: B256) -> Result<()> {

    let input_hex = "0x380f9c38000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000003c00000000000000000c2db89b2bd434ceac6c74fbc0b2ad3a280e66db04a901541000000000000000000000000000000000000000000000000000000000000016000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010d68e7680a9800000000000000000000000000000000000000000000000000000000688b3b7500000000000000000000000000000000000000000000000000000000000004b0000000000000000000000000000000000000000000000000000000000000074800000000000000000000000000000000000000000000000000000000000010e800000000000000000000000000000000000000000000000000000000003567e034a5c9394fb2fd3298ece07c16ec2ed009f6029a360f90f4e93933b55e2184d40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000020bb39b75f048352f913e574a96c291fdb46b8c4bead55bd30bb1b12ecfe0cd999000000000000000000000000000000000000000000000000000000000000005268747470733a2f2f647765622e6c696e6b2f697066732f6261666b726569636d776b33786c78626f7a627035683633787979776f636337646c7474333736686e346d6e6d686b376f6a71646362726b717a69000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001f0181a5737464696edc00100000300500000000ccf32e4f22cc8eccd963ccae0000000000000000000000000000000000000000000000000000000000000000417bae5dcb35129277d3324c610e17928155739ce134b2ffae4838fac2a5f97a8c1ee8bfa94e9aa4fa03fed149a22981572c747498c5be575e07a2b5480f3da84b1c00000000000000000000000000000000000000000000000000000000000000";

    let input_bytes = hex::decode(&input_hex[2..])?;
    let input = Bytes::from(input_bytes);


    let decoded = IBoundlessMarket::submitRequestCall::abi_decode(&input)?;




    println!("Decoded: {:?}", decoded);





    if let Some(tx) = provider.get_transaction_by_hash(tx_hash).await? {
        if tx.to() == Some(MARKET_ADDRESS) {
            println!("成功在 Mempool 中找到{}",tx_hash);

            match IBoundlessMarket::submitRequestCall::abi_decode(tx.input()) {
                Ok(call_data) => {
                    println!("\n=======================================================");
                    println!("🎉 成功在 Mempool 中捕获到新的订单提交！");
                    println!("   - 交易哈希 (Tx Hash): {:?}", tx_hash);
                    println!("   - 订单发起人 (From): {:?}", tx.from());
                    println!("   - 订单 ID (Request ID): 0x{:x}", call_data.request.id);
                    println!("   - 锁定时长 (Lock Timeout): {} seconds", call_data.request.offer.lockTimeout);
                    println!("=======================================================\n");
                }
                Err(_) => {}
            }
            
            // // 使用由 ABI JSON 自动生成的、类型安全的解码器
            // match IBoundlessMarket::ICalls::abi_decode(tx.input(), true) {
            //     Ok(decoded_call) => {
            //         // 解码成功，我们只关心 submitRequest
            //         if let IBoundlessMarket::ICalls::submitRequestCall(call_data) = decoded_call {
            //             println!("\n=======================================================");
            //             println!("🎉 成功在 Mempool 中捕获到新的订单提交！");
            //             println!("   - 交易哈希 (Tx Hash): {:?}", tx_hash);
            //             println!("   - 订单发起人 (From): {:?}", tx.from());
            //             // 现在我们可以轻松访问任何深层嵌套的字段
            //             println!("   - 订单 ID (Request ID): 0x{:x}", call_data.request.id);
            //             println!("   - 锁定时长 (Lock Timeout): {} seconds", call_data.request.offer.lockTimeout);
            //             println!("=======================================================\n");
            //         }
            //     }
            //     Err(e) => {
            //         // 如果解码失败（比如是 lockRequest 或 fulfill），可以选择静默处理或打印日志
            //         // 为了调试，我们先把它打印出来
            //         // println!("\nℹ️  解码交易失败 (可能不是新订单): {} for tx {}", e, tx_hash);
            //     }
            // }
        }
    }
    Ok(())
}
#[tokio::main]
pub async fn main() -> Result<()> {
    // 设置 rustls 的默认加密提供程序
    default_provider()
        .install_default()
        .expect("无法设置 rustls 加密提供程序");
    
    // 使用你自己的 WebSocket 节点 URL
    let ws_url = "wss://base.blockpi.network/v1/ws/69951510051d1ef8e9809575fd95ba9a20748a8d";

    // 创建 WebSocket 连接
    println!("正在连接到 WebSocket 节点: {}", ws_url);
    let ws = WsConnect::new(Url::parse(ws_url)?);
    let provider = Arc::new(ProviderBuilder::new().on_ws(ws).await?);
    println!("连接成功!");

    println!("provider: {:?}", provider);

    // 订阅内存池交易
    println!("订阅 pending 交易...");
    let sub = provider.subscribe_pending_transactions().await?;
    let mut stream = sub.into_stream();
    println!("监听内存池交易中...");





    // 处理交易流
    while let Some(tx_hash) = stream.next().await {
        let provider_clone = provider.clone();
        tokio::spawn(async move {
            match process_transaction(provider_clone, tx_hash).await {
                Ok(_) => {},
                Err(e) => eprintln!("处理交易出错: {:?}", e),
            }
        });
    }

    Ok(())
}















// use serde_json::json;
// use std::time::Duration;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let url = "https://base.blockpi.network/v1/rpc/69951510051d1ef8e9809575fd95ba9a20748a8d";

//     let body = json!({
//         "jsonrpc": "2.0",
//         "method": "txpool_inspect",
//         "params": [],
//         "id": 1
//     });

//     loop {
//         let res = client
//             .post(url)
//             .header("Content-Type", "application/json")
//             .json(&body)
//             .send()
//             .await?;

//         let text = res.text().await?;
//         println!("Response: {}", text);

//         // 每 5 秒请求一次
//         tokio::time::sleep(Duration::from_secs(1)).await;
//     }
// }




