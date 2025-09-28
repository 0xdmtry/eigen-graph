CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS deposits_raw (
                                            id              TEXT         NOT NULL,        -- subgraph Deposit.id (Bytes hex)
                                            token_id        TEXT         NOT NULL,        -- Token.id (address)
                                            token_symbol    TEXT         NOT NULL,        -- Token.symbol
                                            staker          TEXT         NOT NULL,        -- Staker.id
                                            strategy_id     TEXT         NOT NULL,        -- Strategy.id
                                            shares          NUMERIC(78,0) NOT NULL,       -- Deposit.shares (atomic)
    block_number    BIGINT       NOT NULL,
    block_timestamp BIGINT       NOT NULL,        -- unix seconds
    tx_hash         TEXT         NOT NULL
    );

ALTER TABLE deposits_raw DROP CONSTRAINT IF EXISTS deposits_raw_pkey;
ALTER TABLE deposits_raw DROP CONSTRAINT IF EXISTS deposits_raw_tx_hash_key;

DROP INDEX IF EXISTS deposits_raw_tx_hash_key;

ALTER TABLE deposits_raw
    ADD CONSTRAINT deposits_raw_pkey PRIMARY KEY (id, block_timestamp);

CREATE UNIQUE INDEX IF NOT EXISTS ux_deposits_tx_block
    ON deposits_raw (tx_hash, block_timestamp);

SELECT create_hypertable('deposits_raw','block_timestamp',
                         chunk_time_interval => 86400,
                         if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS ix_deposits_token_ts
    ON deposits_raw (token_id, block_timestamp DESC);

CREATE INDEX IF NOT EXISTS ix_deposits_ts
    ON deposits_raw (block_timestamp DESC);
