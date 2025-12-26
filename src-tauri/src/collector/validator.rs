use crate::models::stock::Market;
use crate::models::quote::KLine;
use chrono::{NaiveDate, Utc};
use crate::{Result, AppError};

/// 数据质量评分
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QualityScore {
    Good = 1,      // 良好
    Suspect = 2,   // 可疑
    Error = 3,     // 错误
}

/// 数据验证器
pub struct DataValidator;

impl DataValidator {
    /// 验证股票代码格式
    pub fn validate_code(code: &str) -> Result<()> {
        if code.len() != 6 {
            return Err(AppError::Parse(format!("股票代码长度错误: {}", code)));
        }

        if !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::Parse(format!("股票代码包含非数字字符: {}", code)));
        }

        // 验证市场
        if Market::from_code(code).is_none() {
            return Err(AppError::Parse(format!("无法识别股票代码的市场: {}", code)));
        }

        Ok(())
    }

    /// 验证价格数据
    pub fn validate_price(price: f64, field_name: &str) -> Result<()> {
        if price < 0.0 {
            return Err(AppError::Parse(format!("{}不能为负数: {}", field_name, price)));
        }

        if price > 1_000_000.0 {
            return Err(AppError::Parse(format!("{}异常过高: {}", field_name, price)));
        }

        Ok(())
    }

    /// 验证日期
    pub fn validate_date(date: NaiveDate) -> Result<()> {
        let min_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let max_date = Utc::now().date_naive();

        if date < min_date {
            return Err(AppError::Parse(format!("日期过早: {}", date)));
        }

        if date > max_date {
            return Err(AppError::Parse(format!("日期不能是未来: {}", date)));
        }

        Ok(())
    }

    /// 验证 K 线数据
    pub fn validate_kline(kline: &KLine) -> Result<QualityScore> {
        // 基础验证
        Self::validate_code(&kline.code)?;
        Self::validate_price(kline.open, "开盘价")?;
        Self::validate_price(kline.high, "最高价")?;
        Self::validate_price(kline.low, "最低价")?;
        Self::validate_price(kline.close, "收盘价")?;

        // 逻辑验证: high >= low
        if kline.high < kline.low {
            return Err(AppError::Parse(format!(
                "最高价不能低于最低价: high={}, low={}",
                kline.high, kline.low
            )));
        }

        // 逻辑验证: close 在 [low, high] 范围内
        if kline.close < kline.low || kline.close > kline.high {
            return Err(AppError::Parse(format!(
                "收盘价超出范围: close={}, low={}, high={}",
                kline.close, kline.low, kline.high
            )));
        }

        // 异常检测: 涨跌停
        let change_pct = if kline.low > 0.0 {
            (kline.close - kline.low) / kline.low * 100.0
        } else {
            0.0
        };

        if change_pct.abs() > 20.0 {
            // 科创板、创业板涨跌幅20%
            return Ok(QualityScore::Suspect);
        }

        if change_pct.abs() > 10.0 {
            // 主板涨跌幅10%，可能是涨跌停
            return Ok(QualityScore::Suspect);
        }

        Ok(QualityScore::Good)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_code_valid() {
        assert!(DataValidator::validate_code("000001").is_ok());
        assert!(DataValidator::validate_code("600036").is_ok());
        assert!(DataValidator::validate_code("300001").is_ok());
    }

    #[test]
    fn test_validate_code_invalid() {
        assert!(DataValidator::validate_code("12345").is_err()); // 长度错误
        assert!(DataValidator::validate_code("00000a").is_err()); // 非数字
        assert!(DataValidator::validate_code("123456").is_err()); // 无效市场
    }

    #[test]
    fn test_validate_price() {
        assert!(DataValidator::validate_price(10.5, "测试").is_ok());
        assert!(DataValidator::validate_price(0.0, "测试").is_ok());
        assert!(DataValidator::validate_price(-1.0, "测试").is_err());
    }

    #[test]
    fn test_validate_date() {
        let valid_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        assert!(DataValidator::validate_date(valid_date).is_ok());

        let too_old = NaiveDate::from_ymd_opt(1989, 12, 31).unwrap();
        assert!(DataValidator::validate_date(too_old).is_err());
    }

    #[test]
    fn test_validate_kline_normal() {
        let kline = KLine {
            datetime: Utc::now(),
            code: "000001".to_string(),
            open: 10.0,
            high: 10.5,
            low: 9.8,
            close: 10.2,
            volume: 1000000.0,
            amount: 10200000.0,
        };

        let result = DataValidator::validate_kline(&kline);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), QualityScore::Good);
    }

    #[test]
    fn test_validate_kline_limit_up() {
        let kline = KLine {
            datetime: Utc::now(),
            code: "000001".to_string(),
            open: 10.0,
            high: 11.01,  // 涨停 > 10%
            low: 10.0,
            close: 11.01,
            volume: 1000000.0,
            amount: 11000000.0,
        };

        let result = DataValidator::validate_kline(&kline);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), QualityScore::Suspect); // 涨停标记为可疑
    }
}
