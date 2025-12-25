use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 实时行情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub code: String,
    pub name: String,
    pub price: f64,           // 最新价
    pub preclose: f64,        // 昨收价
    pub open: f64,            // 今开价
    pub high: f64,            // 最高价
    pub low: f64,             // 最低价
    pub volume: f64,          // 成交量 (手)
    pub amount: f64,          // 成交额 (元)
    pub bid: [f64; 5],        // 买盘 5 档价格
    pub bid_vol: [f64; 5],    // 买盘 5 档量
    pub ask: [f64; 5],        // 卖盘 5 档价格
    pub ask_vol: [f64; 5],    // 卖盘 5 档量
    pub timestamp: DateTime<Utc>,
}

impl Quote {
    /// 计算涨跌幅
    pub fn change_pct(&self) -> f64 {
        if self.preclose == 0.0 {
            0.0
        } else {
            (self.price - self.preclose) / self.preclose * 100.0
        }
    }

    /// 计算涨跌额
    pub fn change(&self) -> f64 {
        self.price - self.preclose
    }
}

/// K线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLine {
    pub datetime: DateTime<Utc>,
    pub code: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
}

/// K线周期
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KLinePeriod {
    Minute1,   // 1分钟
    Minute5,   // 5分钟
    Day,       // 日线
    Week,      // 周线
    Month,     // 月线
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_calculations() {
        let quote = Quote {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 10.5,
            preclose: 10.0,
            open: 10.2,
            high: 10.6,
            low: 10.1,
            volume: 100000.0,
            amount: 1050000.0,
            bid: [0.0; 5],
            bid_vol: [0.0; 5],
            ask: [0.0; 5],
            ask_vol: [0.0; 5],
            timestamp: Utc::now(),
        };

        assert_eq!(quote.change(), 0.5);
        assert_eq!(quote.change_pct(), 5.0);
    }
}
