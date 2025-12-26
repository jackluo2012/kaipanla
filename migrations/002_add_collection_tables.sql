-- migrations/002_add_collection_tables.sql
-- 创建数据采集和质量监控表

-- 采集状态表
-- 用于跟踪每日每只股票的数据采集状态
CREATE TABLE IF NOT EXISTS kaipanla.collection_status (
    date Date COMMENT '采集日期',
    code FixedString(6) COMMENT '股票代码',
    status Enum8('success'=1, 'failed'=2, 'pending'=3) COMMENT '采集状态: 成功/失败/待处理',
    retry_count UInt8 DEFAULT 0 COMMENT '重试次数',
    error_message String COMMENT '错误信息（失败时记录）',
    collected_at DateTime COMMENT '采集时间戳',
    updated_at DateTime DEFAULT now() COMMENT '更新时间戳（用于 ReplacingMergeTree 去重）'
) ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (date, code)
COMMENT '数据采集状态表 - 跟踪每只股票的采集进度和结果';

-- 数据质量日志表
-- 用于记录数据质量问题和异常
CREATE TABLE IF NOT EXISTS kaipanla.data_quality_log (
    log_time DateTime COMMENT '日志时间',
    date Date COMMENT '数据日期',
    code FixedString(6) COMMENT '股票代码',
    issue_type Enum8('duplicate'=1, 'gap'=2, 'abnormal'=3, 'missing'=4) COMMENT '问题类型: 重复/缺失/异常/遗漏',
    description String COMMENT '问题描述',
    severity Enum8('info'=1, 'warning'=2, 'error'=3) COMMENT '严重程度: 信息/警告/错误'
) ENGINE = MergeTree()
ORDER BY (log_time, date, code)
COMMENT '数据质量日志表 - 记录数据验证过程中发现的问题';

-- ============================================================
-- 以下为 factor 表优化重建 SQL (可选操作)
-- 注意: ClickHouse 不支持直接修改分区，需要重建表
-- ============================================================

-- 备份数据（如果已有数据）
-- INSERT INTO kaipanla.factor_backup SELECT * FROM kaipanla.factor;

-- 删除旧表（谨慎！）
-- DROP TABLE kaipanla.factor;

-- 重新创建表（带分区和附加字段）
-- CREATE TABLE kaipanla.factor (
--     date Date COMMENT '交易日期',
--     code FixedString(6) COMMENT '股票代码',
--     open Float64 COMMENT '开盘价',
--     high Float64 COMMENT '最高价',
--     low Float64 COMMENT '最低价',
--     close Float64 COMMENT '收盘价',
--     preclose Float64 COMMENT '昨收价',
--     factor Float64 COMMENT '复权因子',
--     volume Float64 COMMENT '成交量',
--     amount Float64 COMMENT '成交额',
--     data_version UInt32 DEFAULT 1 COMMENT '数据版本号（用于追踪数据更新）',
--     data_source Enum('api'=1, 'file'=2, 'manual'=3) DEFAULT 'api' COMMENT '数据来源',
--     quality_score Enum8('good'=1, 'suspect'=2, 'error'=3) DEFAULT 'good' COMMENT '数据质量评分',
--     created_at DateTime DEFAULT now() COMMENT '数据创建时间'
-- ) ENGINE = MergeTree()
-- PARTITION BY toYYYYMM(date)
-- ORDER BY (date, code)
-- COMMENT '日线数据表 - 存储股票日线行情数据（带分区优化）';

-- 恢复数据（如果备份了）
-- INSERT INTO kaipanla.factor
-- SELECT
--     *,
--     1 as data_version,
--     'api' as data_source,
--     1 as quality_score,
--     now() as created_at
-- FROM kaipanla.factor_backup;
