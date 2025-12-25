use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 资金流向数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneyFlow {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub main_inflow: f64,     // 主力流入 (万元)
    pub main_outflow: f64,    // 主力流出 (万元)
    pub retail_inflow: f64,   // 散户流入 (万元)
    pub retail_outflow: f64,  // 散户流出 (万元)
}

impl MoneyFlow {
    /// 计算净流入
    pub fn net_amount(&self) -> f64 {
        self.main_inflow - self.main_outflow + self.retail_inflow - self.retail_outflow
    }

    /// 计算主力净流入
    pub fn main_net(&self) -> f64 {
        self.main_inflow - self.main_outflow
    }

    /// 判断是否主力净流入
    pub fn is_main_inflow(&self) -> bool {
        self.main_net() > 0.0
    }
}

/// 大单交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigTrade {
    pub code: String,
    pub datetime: DateTime<Utc>,
    pub price: f64,
    pub volume: f64,      // 成交量 (手)
    pub amount: f64,      // 成交额 (元)
    pub direction: TradeDirection,
}

/// 交易方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,   // 主动买
    Sell,  // 主动卖
}

impl BigTrade {
    /// 判断是否为大单 (成交额 > 100万)
    pub fn is_big(&self) -> bool {
        self.amount > 1_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_flow_calculations() {
        let flow = MoneyFlow {
            code: "000001".to_string(),
            datetime: Utc::now(),
            main_inflow: 5000.0,
            main_outflow: 3000.0,
            retail_inflow: 2000.0,
            retail_outflow: 4000.0,
        };

        assert_eq!(flow.main_net(), 2000.0);
        assert_eq!(flow.net_amount(), 0.0);
        assert!(flow.is_main_inflow());
    }

    #[test]
    fn test_big_trade_judge() {
        let trade = BigTrade {
            code: "000001".to_string(),
            datetime: Utc::now(),
            price: 10.0,
            volume: 1000.0,
            amount: 1_000_000.0,
            direction: TradeDirection::Buy,
        };

        assert!(!trade.is_big());

        let trade2 = BigTrade {
            amount: 1_000_001.0,
            ..trade
        };

        assert!(trade2.is_big());
    }
}
