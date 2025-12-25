-- migrations/001_init_tables.sql

-- 日线表 (rustdx 原生格式)
CREATE TABLE IF NOT EXISTS kaipanla.factor (
    date Date,
    code FixedString(6),
    open Float64,
    high Float64,
    low Float64,
    close Float64,
    preclose Float64,
    factor Float64,
    volume Float64,
    amount Float64
) ENGINE = MergeTree()
ORDER BY (date, code);

-- 实时行情表
CREATE TABLE IF NOT EXISTS kaipanla.quote_realtime (
    datetime DateTime,
    code FixedString(6),
    price Float64,
    volume Float64,
    amount Float64,
    bids Array(Float64),
    asks Array(Float64)
) ENGINE = MergeTree()
ORDER BY (datetime, code);

-- 龙虎榜表
CREATE TABLE IF NOT EXISTS kaipanla.dragon_tiger (
    date Date,
    code FixedString(6),
    name String,
    reason String,
    broker String,
    buy_amount Float64,
    sell_amount Float64,
    net_amount Float64
) ENGINE = MergeTree()
ORDER BY (date, code);

-- 资金流向表
CREATE TABLE IF NOT EXISTS kaipanla.money_flow (
    datetime DateTime,
    code FixedString(6),
    main_inflow Float64,
    main_outflow Float64,
    retail_inflow Float64,
    retail_outflow Float64,
    net_amount Float64
) ENGINE = MergeTree()
ORDER BY (datetime, code);