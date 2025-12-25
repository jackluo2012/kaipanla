use crate::models::{MoneyFlow, TradeDirection};
use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};

/// 资金监控服务
pub struct MoneyFlowService;

impl MoneyFlowService {
    /// 创建新的资金监控服务
    pub fn new() -> Self {
        Self
    }

    /// 分析单笔交易的资金流向
    ///
    /// # 参数
    /// * `code` - 股票代码
    /// * `price` - 成交价格
    /// * `volume` - 成交量（手）
    /// * `direction` - 交易方向
    /// * `timestamp` - 时间戳
    ///
    /// # 返回
    /// 资金流向数据（万元）
    pub fn analyze_trade(
        &self,
        code: &str,
        price: f64,
        volume: f64,
        direction: TradeDirection,
        timestamp: DateTime<Utc>,
    ) -> MoneyFlow {
        let amount = price * volume * 100.0; // 成交额（元）

        // 判断是否为大单（成交额 > 100万）
        let is_big = amount > 1_000_000.0;

        // 根据交易方向和单笔大小分类资金流向
        let (main_inflow, main_outflow, retail_inflow, retail_outflow) = match direction {
            TradeDirection::Buy if is_big => (amount / 10_000.0, 0.0, 0.0, 0.0),
            TradeDirection::Sell if is_big => (0.0, amount / 10_000.0, 0.0, 0.0),
            TradeDirection::Buy => (0.0, 0.0, amount / 10_000.0, 0.0),
            TradeDirection::Sell => (0.0, 0.0, 0.0, amount / 10_000.0),
        };

        MoneyFlow {
            code: code.to_string(),
            datetime: timestamp,
            main_inflow,
            main_outflow,
            retail_inflow,
            retail_outflow,
        }
    }

    /// 聚合一段时间内的资金流向
    ///
    /// # 参数
    /// * `flows` - 资金流向数据列表
    ///
    /// # 返回
    /// 聚合后的资金流向数据
    pub fn aggregate_money_flow(
        &self,
        flows: Vec<MoneyFlow>,
    ) -> Result<MoneyFlow> {
        if flows.is_empty() {
            return Err(AppError::NotFound("没有资金流向数据".to_string()));
        }

        let code = flows[0].code.clone();
        let datetime = flows[0].datetime;

        // 聚合各项资金流向
        let main_inflow: f64 = flows.iter().map(|f| f.main_inflow).sum();
        let main_outflow: f64 = flows.iter().map(|f| f.main_outflow).sum();
        let retail_inflow: f64 = flows.iter().map(|f| f.retail_inflow).sum();
        let retail_outflow: f64 = flows.iter().map(|f| f.retail_outflow).sum();

        Ok(MoneyFlow {
            code,
            datetime,
            main_inflow,
            main_outflow,
            retail_inflow,
            retail_outflow,
        })
    }

    /// 获取当日资金流向（占位实现）
    ///
    /// # 参数
    /// * `code` - 股票代码
    ///
    /// # 返回
    /// 当日资金流向数据
    pub async fn get_daily_money_flow(&self, code: &str) -> Result<MoneyFlow> {
        tracing::debug!("获取股票 {} 当日资金流向", code);

        // TODO: 从 ClickHouse 查询真实数据
        Ok(MoneyFlow {
            code: code.to_string(),
            datetime: Utc::now(),
            main_inflow: 5000.0,
            main_outflow: 3000.0,
            retail_inflow: 2000.0,
            retail_outflow: 4000.0,
        })
    }
}

impl Default for MoneyFlowService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_big_trade_buy() {
        let service = MoneyFlowService::new();

        // 测试大单买入：10元 * 10000手 = 1000万 > 100万
        let flow = service.analyze_trade(
            "000001",
            10.0,
            10000.0,
            TradeDirection::Buy,
            Utc::now(),
        );

        assert_eq!(flow.code, "000001");
        assert_eq!(flow.main_inflow, 10_000_000.0 / 10_000.0); // 1000万 ÷ 10000 = 1000万元
        assert_eq!(flow.main_outflow, 0.0);
        assert_eq!(flow.retail_inflow, 0.0);
        assert_eq!(flow.retail_outflow, 0.0);
        assert!(flow.is_main_inflow());
    }

    #[test]
    fn test_analyze_big_trade_sell() {
        let service = MoneyFlowService::new();

        // 测试大单卖出：10元 * 20000手 = 2000万 > 100万
        let flow = service.analyze_trade(
            "600036",
            10.0,
            20000.0,
            TradeDirection::Sell,
            Utc::now(),
        );

        assert_eq!(flow.code, "600036");
        assert_eq!(flow.main_inflow, 0.0);
        assert_eq!(flow.main_outflow, 20_000_000.0 / 10_000.0); // 2000万 ÷ 10000 = 2000万元
        assert_eq!(flow.retail_inflow, 0.0);
        assert_eq!(flow.retail_outflow, 0.0);
        assert!(!flow.is_main_inflow());
    }

    #[test]
    fn test_analyze_small_trade_buy() {
        let service = MoneyFlowService::new();

        // 测试小单买入：10元 * 100手 = 10万 < 100万
        let flow = service.analyze_trade(
            "000001",
            10.0,
            100.0,
            TradeDirection::Buy,
            Utc::now(),
        );

        assert_eq!(flow.code, "000001");
        assert_eq!(flow.main_inflow, 0.0);
        assert_eq!(flow.main_outflow, 0.0);
        assert_eq!(flow.retail_inflow, 100_000.0 / 10_000.0); // 10万 ÷ 10000 = 10万元
        assert_eq!(flow.retail_outflow, 0.0);
    }

    #[test]
    fn test_analyze_small_trade_sell() {
        let service = MoneyFlowService::new();

        // 测试小单卖出：10元 * 50手 = 5万 < 100万
        let flow = service.analyze_trade(
            "600036",
            10.0,
            50.0,
            TradeDirection::Sell,
            Utc::now(),
        );

        assert_eq!(flow.code, "600036");
        assert_eq!(flow.main_inflow, 0.0);
        assert_eq!(flow.main_outflow, 0.0);
        assert_eq!(flow.retail_inflow, 0.0);
        assert_eq!(flow.retail_outflow, 50_000.0 / 10_000.0); // 5万 ÷ 10000 = 5万元
    }

    #[test]
    fn test_aggregate_money_flow() {
        let service = MoneyFlowService::new();

        let flows = vec![
            MoneyFlow {
                code: "000001".to_string(),
                datetime: Utc::now(),
                main_inflow: 1000.0,
                main_outflow: 500.0,
                retail_inflow: 200.0,
                retail_outflow: 300.0,
            },
            MoneyFlow {
                code: "000001".to_string(),
                datetime: Utc::now(),
                main_inflow: 800.0,
                main_outflow: 600.0,
                retail_inflow: 150.0,
                retail_outflow: 250.0,
            },
        ];

        let aggregated = service.aggregate_money_flow(flows).unwrap();

        assert_eq!(aggregated.code, "000001");
        assert_eq!(aggregated.main_inflow, 1800.0); // 1000 + 800
        assert_eq!(aggregated.main_outflow, 1100.0); // 500 + 600
        assert_eq!(aggregated.retail_inflow, 350.0); // 200 + 150
        assert_eq!(aggregated.retail_outflow, 550.0); // 300 + 250
        assert_eq!(aggregated.main_net(), 700.0); // 1800 - 1100
        assert_eq!(aggregated.net_amount(), 500.0); // 700 - 200
    }

    #[test]
    fn test_aggregate_empty_flows() {
        let service = MoneyFlowService::new();

        let result = service.aggregate_money_flow(vec![]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[test]
    fn test_aggregate_single_flow() {
        let service = MoneyFlowService::new();

        let flow = MoneyFlow {
            code: "000001".to_string(),
            datetime: Utc::now(),
            main_inflow: 1000.0,
            main_outflow: 500.0,
            retail_inflow: 200.0,
            retail_outflow: 300.0,
        };

        let aggregated = service.aggregate_money_flow(vec![flow.clone()]).unwrap();

        assert_eq!(aggregated.code, flow.code);
        assert_eq!(aggregated.main_inflow, 1000.0);
        assert_eq!(aggregated.main_outflow, 500.0);
        assert_eq!(aggregated.retail_inflow, 200.0);
        assert_eq!(aggregated.retail_outflow, 300.0);
    }

    #[tokio::test]
    async fn test_get_daily_money_flow() {
        let service = MoneyFlowService::new();

        let flow = service.get_daily_money_flow("000001").await.unwrap();

        assert_eq!(flow.code, "000001");
        assert_eq!(flow.main_inflow, 5000.0);
        assert_eq!(flow.main_outflow, 3000.0);
        assert_eq!(flow.retail_inflow, 2000.0);
        assert_eq!(flow.retail_outflow, 4000.0);
        assert_eq!(flow.main_net(), 2000.0);
        assert_eq!(flow.net_amount(), 0.0);
    }

    #[test]
    fn test_money_flow_service_default() {
        let service = MoneyFlowService::default();
        // 测试默认创建
        let flow = service.analyze_trade(
            "000001",
            10.0,
            1000.0,
            TradeDirection::Buy,
            Utc::now(),
        );
        assert_eq!(flow.code, "000001");
    }
}
