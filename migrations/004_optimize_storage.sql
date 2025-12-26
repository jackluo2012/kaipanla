-- ClickHouse 存储优化
-- 版本: v0.4.0
-- 日期: 2025-12-26

-- ============================================================
-- 1. factor 表优化
-- ============================================================

-- 添加数据质量字段
ALTER TABLE kaipanla.factor
ADD COLUMN IF NOT EXISTS data_version UInt32 DEFAULT 1,
ADD COLUMN IF NOT EXISTS data_source Enum8('api'=1, 'file'=2, 'manual'=3) DEFAULT 'api',
ADD COLUMN IF NOT EXISTS quality_score Enum8('good'=1, 'suspect'=2, 'error'=3) DEFAULT 'good',
ADD COLUMN IF NOT EXISTS created_at DateTime DEFAULT now();

-- 添加二级索引加速查询
-- 注意：ClickHouse 的索引需要在表创建时定义，这里演示语法
-- 实际生产环境建议重新创建表并导入数据

-- ============================================================
-- 2. 查询优化视图
-- ============================================================

-- 最近30天数据视图（常用查询）
CREATE VIEW IF NOT EXISTS kaipanla.v_recent_30days AS
SELECT
    code,
    argMax(datetime, datetime) as latest_datetime,
    argMax(open, datetime) as latest_open,
    argMax(high, datetime) as latest_high,
    argMax(low, datetime) as latest_low,
    argMax(close, datetime) as latest_close,
    argMax(volume, datetime) as latest_volume,
    argMax(amount, datetime) as latest_amount
FROM kaipanla.factor
WHERE datetime >= now() - INTERVAL 30 DAY
GROUP BY code;

-- 全市场最新快照视图
CREATE VIEW IF NOT EXISTS kaipanla.v_latest_snapshot AS
SELECT
    code,
    toDateTime(toMaxZone(datetime, 'Asia/Shanghai')) as snapshot_time,
    any(open) as open,
    any(high) as high,
    any(low) as low,
    any(close) as close,
    sum(volume) as volume,
    sum(amount) as amount,
    any(quality_score) as quality_score
FROM kaipanla.factor
WHERE datetime >= today() - INTERVAL 7 DAY
GROUP BY code
HAVING snapshot_time >= now() - INTERVAL 1 HOUR;

-- ============================================================
-- 3. 数据质量统计
-- ============================================================

-- 每日数据质量统计
CREATE MATERIALIZED VIEW IF NOT EXISTS kaipanla.mv_daily_quality_stats
ENGINE = SummingMergeTree()
ORDER BY (date, quality_score)
POPULATE
AS SELECT
    toDate(datetime) as date,
    quality_score,
    count() as record_count,
    countDistinct(code) as unique_stocks
FROM kaipanla.factor
GROUP BY date, quality_score;

-- ============================================================
-- 4. 数据保留策略（通过 TTL 实现）
-- ============================================================

-- factor 表：永久保留历史数据
-- 实时采集日志：保留30天
-- 注意：TTL 需要在表创建时设置，这里演示语法
-- ALTER TABLE kaipanla.factor MODIFY TTL datetime + INTERVAL 3 YEAR;

-- collection_status 表：保留90天
-- ALTER TABLE kaipanla.collection_status MODIFY TTL updated_at + INTERVAL 90 DAY;

-- data_quality_log 表：保留180天
-- ALTER TABLE kaipanla.data_quality_log MODIFY TTL log_time + INTERVAL 180 DAY;

-- ============================================================
-- 5. 性能优化设置
-- ============================================================

-- 设置异步插入模式（提高写入性能）
-- SET async_insert = 1;
-- SET wait_for_async_insert = 0;

-- 设置写入并发数
-- SET max_insert_threads = 4;
-- SET max_insert_block_size = 1048576;

-- ============================================================
-- 6. 监控和统计查询
-- ============================================================

-- 表统计信息
-- SELECT
--     database,
--     table,
--     formatReadableSize(sum(bytes)) as size,
--     sum(rows) as rows,
--     count() as parts
-- FROM system.parts
-- WHERE active AND database = 'kaipanla'
-- GROUP BY database, table;

-- 查询最慢的查询
-- SELECT
--     query,
--     query_duration_ms,
--     read_rows,
--     written_rows
-- FROM system.query_log
-- WHERE type = 'QueryFinish'
-- ORDER BY query_duration_ms DESC
-- LIMIT 10;
