// crates/broker/src/ws_provider.rs

use std::sync::Arc;

use alloy::{
    network::Ethereum,
    providers::{Provider, ProviderBuilder},
};
// 移除了不再需要的 WsConnect
use anyhow::Result;
use tokio::sync::OnceCell;
use url::Url; // Url 仍然需要，因为 ws_url 需要被解析

use crate::market_monitor::MarketMonitorErr;

static WS_PROVIDER: OnceCell<Arc<dyn Provider<Ethereum> + Send + Sync>> = OnceCell::const_new();

pub async fn get_ws_provider() -> Result<Arc<dyn Provider<Ethereum> + Send + Sync>, MarketMonitorErr> {
    WS_PROVIDER
        .get_or_try_init(|| async {
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("无法设置 rustls 加密提供程序");

            let ws_url = "wss://base.blockpi.network/v1/ws/69951510051d1ef8e9809575fd95ba9a20748a8d";
            println!("正在连接到 WebSocket 节点: {}", ws_url);
            
            // --- 核心修正 1: .connect() 方法接收 &str 类型的 URL ---
            // 我们直接将 ws_url 传递给 .connect()，不再手动创建 WsConnect 对象。
            let provider = ProviderBuilder::new()
                .connect(ws_url) // 这里传入 URL 字符串
                .await
                .map_err(|e| MarketMonitorErr::UnexpectedErr(anyhow::anyhow!(e)))?;

            // --- 核心修正 2: 显式进行类型转换 ---
            // 创建一个明确指定了 trait object 类型的变量，
            // 编译器会自动将右边的具体类型 Arc<FillProvider<...>> 转换为左边的抽象类型。
            let provider_as_trait: Arc<dyn Provider<Ethereum> + Send + Sync> = Arc::new(provider);

            Ok(provider_as_trait)
        })
        .await
        .cloned()
}
