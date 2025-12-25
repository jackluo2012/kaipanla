use serde::{Deserialize, Serialize};

/// 股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub code: String,      // 股票代码 (6位)
    pub name: String,      // 股票名称
    pub market: Market,    // 所属市场
}

/// 市场类型
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Market {
    SZ,  // 深交所
    SH,  // 上交所
    BJ,  // 北交所
}

impl Market {
    /// 从股票代码判断市场
    pub fn from_code(code: &str) -> Option<Self> {
        if code.len() != 6 {
            return None;
        }

        let first = &code[0..2];
        match first {
            "00" | "30" => Some(Market::SZ),
            "60" | "68" => Some(Market::SH),
            "43" | "83" | "87" => Some(Market::BJ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_from_code() {
        assert_eq!(Market::from_code("000001"), Some(Market::SZ));
        assert_eq!(Market::from_code("300001"), Some(Market::SZ));
        assert_eq!(Market::from_code("600000"), Some(Market::SH));
        assert_eq!(Market::from_code("688001"), Some(Market::SH));
        assert_eq!(Market::from_code("430001"), Some(Market::BJ));
        assert_eq!(Market::from_code("123456"), None);
        assert_eq!(Market::from_code("12345"), None);
    }
}
