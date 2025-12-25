use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 集合竞价数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub code: String,
    pub name: String,
    pub price: f64,           // 竞价价格
    pub preclose: f64,        // 昨收价
    pub volume: f64,          // 竞成交量（手）
    pub amount: f64,          // 竞价金额（元）
    pub timestamp: DateTime<Utc>,
}

impl Auction {
    /// 计算竞价涨跌幅
    pub fn change_pct(&self) -> f64 {
        if self.preclose == 0.0 {
            0.0
        } else {
            (self.price - self.preclose) / self.preclose * 100.0
        }
    }

    /// 计算量比（简化版）
    pub fn volume_ratio(&self, avg_volume: f64) -> f64 {
        if avg_volume == 0.0 {
            0.0
        } else {
            self.volume / avg_volume
        }
    }

    /// 判断是否异动
    pub fn is_anomaly(&self) -> bool {
        let change = self.change_pct().abs();
        change > 5.0
    }
}

/// 竞价异动记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionAnomaly {
    pub code: String,
    pub name: String,
    pub change_pct: f64,
    pub volume_ratio: f64,
    pub reason: AnomalyReason,
}

/// 异动原因
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyReason {
    /// 高开
    HighOpen,
    /// 低开
    LowOpen,
    /// 放量
    HighVolume,
    /// 缩量
    LowVolume,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_change_pct() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 10.0,
            volume: 10000.0,
            amount: 105000.0,
            timestamp: Utc::now(),
        };

        assert_eq!(auction.change_pct(), 5.0);
    }

    #[test]
    fn test_auction_is_anomaly() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.6,
            preclose: 10.0,
            volume: 10000.0,
            amount: 106000.0,
            timestamp: Utc::now(),
        };

        assert!(auction.is_anomaly());
    }

    #[test]
    fn test_auction_volume_ratio() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 10.0,
            volume: 20000.0,
            amount: 210000.0,
            timestamp: Utc::now(),
        };

        let ratio = auction.volume_ratio(10000.0);
        assert_eq!(ratio, 2.0);
    }

    #[test]
    fn test_auction_not_anomaly() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.3,
            preclose: 10.0,
            volume: 10000.0,
            amount: 103000.0,
            timestamp: Utc::now(),
        };

        assert!(!auction.is_anomaly());
    }

    #[test]
    fn test_auction_zero_preclose() {
        let auction = Auction {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 0.0,
            volume: 10000.0,
            amount: 105000.0,
            timestamp: Utc::now(),
        };

        assert_eq!(auction.change_pct(), 0.0);
    }
}
