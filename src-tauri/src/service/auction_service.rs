use crate::models::auction::{Auction, AuctionAnomaly, AnomalyReason};
use crate::Result;

/// 竞价分析服务
pub struct AuctionService;

impl AuctionService {
    /// 创建新的竞价分析服务
    pub fn new() -> Self {
        Self
    }

    /// 获取集合竞价数据（占位实现）
    pub async fn get_auction_data(&self) -> Result<Vec<Auction>> {
        tracing::debug!("获取集合竞价数据");

        // TODO: 从通达信获取竞价数据
        Ok(vec![])
    }

    /// 分析竞价异动
    pub fn analyze_anomalies(
        &self,
        auctions: Vec<Auction>,
    ) -> Result<Vec<AuctionAnomaly>> {
        let mut anomalies = Vec::new();

        for auction in auctions {
            let change_pct = auction.change_pct();
            let volume_ratio = 1.0; // TODO: 计算实际量比

            // 判断异动
            if change_pct > 5.0 {
                anomalies.push(AuctionAnomaly {
                    code: auction.code.clone(),
                    name: auction.name.clone(),
                    change_pct,
                    volume_ratio,
                    reason: AnomalyReason::HighOpen,
                });
            } else if change_pct < -5.0 {
                anomalies.push(AuctionAnomaly {
                    code: auction.code.clone(),
                    name: auction.name.clone(),
                    change_pct,
                    volume_ratio,
                    reason: AnomalyReason::LowOpen,
                });
            }
        }

        Ok(anomalies)
    }

    /// 识别龙头股
    pub fn identify_leaders(
        &self,
        anomalies: Vec<AuctionAnomaly>,
    ) -> Vec<String> {
        let mut leaders: Vec<(String, f64)> = anomalies
            .into_iter()
            .filter(|a| matches!(a.reason, AnomalyReason::HighOpen))
            .map(|a| (a.code, a.change_pct))
            .collect();

        // 按涨幅排序
        leaders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        leaders.into_iter().map(|(code, _)| code).collect()
    }

    /// 获取竞价异动列表（占位实现）
    pub async fn get_anomaly_list(&self) -> Result<Vec<AuctionAnomaly>> {
        let auctions = self.get_auction_data().await?;
        self.analyze_anomalies(auctions)
    }

    /// 获取龙头股列表（占位实现）
    pub async fn get_leader_list(&self) -> Result<Vec<String>> {
        let auctions = self.get_auction_data().await?;
        let anomalies = self.analyze_anomalies(auctions)?;
        Ok(self.identify_leaders(anomalies))
    }
}

impl Default for AuctionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_service_creation() {
        let service = AuctionService::new();
        let result = service.analyze_anomalies(vec![]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_analyze_anomalies_high_open() {
        let service = AuctionService::new();

        let auctions = vec![
            Auction {
                code: "000001".to_string(),
                name: "平安银行".to_string(),
                price: 10.6,
                preclose: 10.0,
                volume: 10000.0,
                amount: 106000.0,
                timestamp: Utc::now(),
            },
        ];

        let result = service.analyze_anomalies(auctions).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "000001");
        assert!(result[0].change_pct > 5.0);
        assert!(matches!(result[0].reason, AnomalyReason::HighOpen));
    }

    #[test]
    fn test_analyze_anomalies_low_open() {
        let service = AuctionService::new();

        let auctions = vec![
            Auction {
                code: "000001".to_string(),
                name: "平安银行".to_string(),
                price: 9.4,
                preclose: 10.0,
                volume: 10000.0,
                amount: 94000.0,
                timestamp: Utc::now(),
            },
        ];

        let result = service.analyze_anomalies(auctions).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "000001");
        assert!(result[0].change_pct < -5.0);
        assert!(matches!(result[0].reason, AnomalyReason::LowOpen));
    }

    #[test]
    fn test_analyze_anomalies_no_anomaly() {
        let service = AuctionService::new();

        let auctions = vec![
            Auction {
                code: "000001".to_string(),
                name: "平安银行".to_string(),
                price: 10.3,
                preclose: 10.0,
                volume: 10000.0,
                amount: 103000.0,
                timestamp: Utc::now(),
            },
        ];

        let result = service.analyze_anomalies(auctions).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_identify_leaders() {
        let service = AuctionService::new();

        let anomalies = vec![
            AuctionAnomaly {
                code: "000001".to_string(),
                name: "平安银行".to_string(),
                change_pct: 6.0,
                volume_ratio: 1.5,
                reason: AnomalyReason::HighOpen,
            },
            AuctionAnomaly {
                code: "600036".to_string(),
                name: "招商银行".to_string(),
                change_pct: 8.0,
                volume_ratio: 2.0,
                reason: AnomalyReason::HighOpen,
            },
            AuctionAnomaly {
                code: "000002".to_string(),
                name: "万科A".to_string(),
                change_pct: -6.0,
                volume_ratio: 1.2,
                reason: AnomalyReason::LowOpen,
            },
        ];

        let leaders = service.identify_leaders(anomalies);
        assert_eq!(leaders.len(), 2);
        assert_eq!(leaders[0], "600036"); // 涨幅最高
        assert_eq!(leaders[1], "000001");
    }

    #[tokio::test]
    async fn test_get_auction_data() {
        let service = AuctionService::new();
        let result = service.get_auction_data().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // 占位实现返回空
    }

    #[tokio::test]
    async fn test_get_anomaly_list() {
        let service = AuctionService::new();
        let result = service.get_anomaly_list().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_leader_list() {
        let service = AuctionService::new();
        let result = service.get_leader_list().await;
        assert!(result.is_ok());
    }
}
