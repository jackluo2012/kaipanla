use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// 龙虎榜记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonTiger {
    pub date: NaiveDate,
    pub code: String,
    pub name: String,
    pub reason: DragonReason,  // 上榜原因
    pub broker: String,        // 营业部名称
    pub buy_amount: f64,       // 买入金额（万元）
    pub sell_amount: f64,      // 卖出金额（万元）
    pub net_amount: f64,       // 净买入（万元）
}

impl DragonTiger {
    /// 计算净买入
    pub fn net(&self) -> f64 {
        self.buy_amount - self.sell_amount
    }
}

/// 上榜原因
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragonReason {
    /// 日涨跌幅偏离值达7%
    UpLimit,
    /// 日跌跌幅偏离值达7%
    DownLimit,
    /// 日换手率达20%
    HighTurnover,
    /// 连续三个交易日内涨跌幅偏离值累计达20%
    ThreeDayUp,
    /// 日价格涨幅偏离值达7%
    PriceUp,
    /// 其他
    Other(String),
}

/// 营业部统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerStats {
    pub broker: String,
    pub appear_count: i32,    // 上榜次数
    pub total_buy: f64,       // 总买入（万元）
    pub total_sell: f64,      // 总卖出（万元）
    pub total_net: f64,       // 总净买入（万元）
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_tiger_net() {
        let dt = DragonTiger {
            date: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            reason: DragonReason::UpLimit,
            broker: "东方财富拉萨营业部".to_string(),
            buy_amount: 5000.0,
            sell_amount: 2000.0,
            net_amount: 0.0,
        };

        assert_eq!(dt.net(), 3000.0);
    }

    #[test]
    fn test_dragon_tiger_serialize() {
        let dt = DragonTiger {
            date: NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            reason: DragonReason::UpLimit,
            broker: "东方财富拉萨营业部".to_string(),
            buy_amount: 5000.0,
            sell_amount: 2000.0,
            net_amount: 0.0,
        };

        let json = serde_json::to_string(&dt).unwrap();
        assert!(json.contains("000001"));
        assert!(json.contains("平安银行"));
    }

    #[test]
    fn test_dragon_reason_other() {
        let reason = DragonReason::Other("特殊原因".to_string());
        let json = serde_json::to_string(&reason).unwrap();
        assert!(json.contains("特殊原因"));
    }

    #[test]
    fn test_broker_stats() {
        let stats = BrokerStats {
            broker: "东方财富拉萨营业部".to_string(),
            appear_count: 100,
            total_buy: 100000.0,
            total_sell: 80000.0,
            total_net: 20000.0,
        };

        assert_eq!(stats.broker, "东方财富拉萨营业部");
        assert_eq!(stats.appear_count, 100);
        assert_eq!(stats.total_net, 20000.0);
    }
}
