// src/ws_provider.rs

use std::sync::Arc;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::WsConnect;
use url::Url;
use tokio::sync::OnceCell; // 注意：OnceCell 比 OnceLock 更友好
use crate::market_monitor::MarketMonitorErr;

static WS_PROVIDER: OnceCell<Arc<_>> = OnceCell::const_new();

pub async fn get_ws_provider() -> Result<Arc<impl Provider>, MarketMonitorErr> {
    WS_PROVIDER
        .get_or_try_init(|| async {
            // 设置 rustls 加密提供程序
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("无法设置 rustls 加密提供程序");

            let ws_url = "wss://base.blockpi.network/v1/ws/69951510051d1ef8e9809575fd95ba9a20748a8d";
            println!("正在连接到 WebSocket 节点: {}", ws_url);

            let ws = WsConnect::new(
                Url::parse(ws_url)
                    .map_err(|e| MarketMonitorErr::UnexpectedErr(anyhow::anyhow!(e)))?,
            );

            let provider = ProviderBuilder::new()
                .on_ws(ws)
                .await
                .map_err(|e| MarketMonitorErr::UnexpectedErr(anyhow::anyhow!(e)))?;

            Ok::<_, MarketMonitorErr>(Arc::new(provider))
        })
        .await
        .cloned()
}
