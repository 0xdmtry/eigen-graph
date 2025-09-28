CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS deposits_raw (
                                            id               TEXT        PRIMARY KEY,     -- subgraph Deposit.id (Bytes hex)
                                            token_id         TEXT        NOT NULL,        -- Token.id (address)
                                            token_symbol     TEXT        NOT NULL,        -- Token.symbol (for quick filtering)
                                            staker           TEXT        NOT NULL,        -- Staker.id (address)
                                            strategy_id      TEXT        NOT NULL,        -- Strategy.id
                                            shares           NUMERIC(78,0) NOT NULL,      -- Deposit.shares (atomic)
    block_number     BIGINT      NOT NULL,
    block_timestamp  BIGINT      NOT NULL,        -- unix seconds
    tx_hash          TEXT        NOT NULL UNIQUE  -- guard idempotency
    );

SELECT create_hypertable('deposits_raw','block_timestamp',
                         chunk_time_interval => 86400,
                         if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS ix_deposits_token_ts
    ON deposits_raw (token_id, block_timestamp DESC);

CREATE INDEX IF NOT EXISTS ix_deposits_ts
    ON deposits_raw (block_timestamp DESC);
