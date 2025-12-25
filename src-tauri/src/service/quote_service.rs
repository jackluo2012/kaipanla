use crate::collector::tdx::TdxClient;
use crate::models::{Quote, Stock};
use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 实时行情服务
pub struct QuoteService {
    tdx_client: TdxClient,
    cache: Arc<RwLock<HashMap<String, Quote>>>,
}

impl QuoteService {
    /// 创建新的行情服务
    pub fn new(tdx_client: TdxClient) -> Self {
        Self {
            tdx_client,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 获取单只股票实时行情
    pub async fn get_quote(&self, code: &str) -> Result<Quote> {
        // 先从缓存查找
        {
            let cache = self.cache.read().await;
            if let Some(quote) = cache.get(code) {
                return Ok(quote.clone());
            }
        }

        // 缓存未命中，获取实时数据（占位实现）
        let quote = self.fetch_quote(code).await?;

        // 更新缓存
        {
            let mut cache = self.cache.write().await;
            cache.insert(code.to_string(), quote.clone());
        }

        Ok(quote)
    }

    /// 批量获取行情
    pub async fn get_quotes(&self, codes: &[String]) -> Result<Vec<Quote>> {
        let mut quotes = Vec::new();

        for code in codes {
            match self.get_quote(code).await {
                Ok(quote) => quotes.push(quote),
                Err(e) => {
                    tracing::warn!("获取股票 {} 行情失败: {}", code, e);
                }
            }
        }

        Ok(quotes)
    }

    /// 获取股票列表
    pub async fn get_stock_list(&self) -> Result<Vec<Stock>> {
        // TODO: 从数据库或通达信获取
        Ok(vec![])
    }

    /// 从通达信获取实时行情（占位实现）
    async fn fetch_quote(&self, code: &str) -> Result<Quote> {
        tracing::debug!("获取股票 {} 实时行情", code);

        // TODO: 使用 rustdx API 获取真实数据
        // 这里返回模拟数据用于测试
        Ok(Quote {
            code: code.to_string(),
            name: "测试股票".to_string(),
            price: 10.5,
            preclose: 10.0,
            open: 10.2,
            high: 10.6,
            low: 10.1,
            volume: 100000.0,
            amount: 1050000.0,
            bid: [10.49, 10.48, 10.47, 10.46, 10.45],
            bid_vol: [1000.0, 2000.0, 3000.0, 4000.0, 5000.0],
            ask: [10.51, 10.52, 10.53, 10.54, 10.55],
            ask_vol: [1000.0, 2000.0, 3000.0, 4000.0, 5000.0],
            timestamp: chrono::Utc::now(),
        })
    }

    /// 启动实时行情推送任务
    pub async fn start_update_task(&self) -> Result<()> {
        tracing::info!("启动实时行情更新任务");

        // TODO: 定时更新缓存
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quote_service_creation() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);
        assert_eq!(service.cache.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_get_quote() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);

        let quote = service.get_quote("000001").await.unwrap();
        assert_eq!(quote.code, "000001");
        assert_eq!(quote.price, 10.5);
    }

    #[tokio::test]
    async fn test_get_quotes() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);

        let codes = vec!["000001".to_string(), "600036".to_string()];
        let quotes = service.get_quotes(&codes).await.unwrap();

        assert_eq!(quotes.len(), 2);
        assert_eq!(quotes[0].code, "000001");
        assert_eq!(quotes[1].code, "600036");
    }

    #[tokio::test]
    async fn test_quote_cache() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);

        // 第一次获取
        let quote1 = service.get_quote("000001").await.unwrap();

        // 第二次获取应该从缓存返回
        let quote2 = service.get_quote("000001").await.unwrap();

        // 验证缓存大小为1
        assert_eq!(service.cache.read().await.len(), 1);

        // 两次获取的结果应该相同（因为是同一个缓存对象）
        assert_eq!(quote1.code, quote2.code);
        assert_eq!(quote1.price, quote2.price);
    }

    #[tokio::test]
    async fn test_get_stock_list() {
        let tdx_client = TdxClient::new(vec!["127.0.0.1:7709".to_string()]);
        let service = QuoteService::new(tdx_client);

        let stocks = service.get_stock_list().await.unwrap();
        assert_eq!(stocks.len(), 0); // 占位实现返回空列表
    }
}
