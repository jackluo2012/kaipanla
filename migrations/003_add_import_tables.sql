-- 历史数据导入进度表
CREATE TABLE IF NOT EXISTS kaipanla.import_progress (
    id UInt32 DEFAULT 0,
    stage Enum8('idle'=0, 'importing_recent'=1, 'importing_history'=2, 'completed'=3, 'failed'=4, 'cancelled'=5) DEFAULT 'idle',
    total_stocks UInt32 DEFAULT 0,
    imported_stocks UInt32 DEFAULT 0,
    total_batches UInt32 DEFAULT 0,
    imported_batches UInt32 DEFAULT 0,
    current_code FixedString(6),
    start_date Date,
    end_date Date,
    error_count UInt32 DEFAULT 0,
    created_at DateTime DEFAULT now(),
    updated_at DateTime DEFAULT now()
) ENGINE = ReplacingMergeTree(updated_at)
ORDER BY (id);

-- 插入初始记录
INSERT INTO kaipanla.import_progress (id) VALUES (0);
