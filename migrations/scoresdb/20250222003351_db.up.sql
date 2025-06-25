-- Table for miner stats --
CREATE TABLE miner_stats (
    miner_uid INTEGER PRIMARY KEY,
    alpha FLOAT DEFAULT 500.0,
    beta FLOAT DEFAULT 1000.0,
    challenge_successes INTEGER DEFAULT 0,
    challenge_attempts INTEGER DEFAULT 0,
    retrieval_successes INTEGER DEFAULT 0,
    retrieval_attempts INTEGER DEFAULT 0,
    store_successes INTEGER DEFAULT 0,
    store_attempts INTEGER DEFAULT 0,
    total_successes INTEGER DEFAULT 0
);

WITH RECURSIVE numbers AS (
    SELECT 0 AS value
    UNION ALL
    SELECT value + 1
    FROM numbers
    WHERE value < 255
)

INSERT INTO miner_stats (miner_uid)
SELECT value FROM numbers;
