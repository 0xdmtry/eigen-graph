CREATE TABLE IF NOT EXISTS operators_snapshot (
                                                  operator_id            TEXT PRIMARY KEY,
                                                  avs_count              INT NOT NULL,
                                                  strategy_count         INT NOT NULL,
                                                  slashing_count         INT NOT NULL,
                                                  last_slash_at          BIGINT NULL,
                                                  last_update_block_ts   BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_opsnap_last_update
    ON operators_snapshot (last_update_block_ts);

CREATE TABLE IF NOT EXISTS operator_strategy (
                                                 operator_id     TEXT NOT NULL,
                                                 strategy_id     TEXT NOT NULL,
                                                 token_id        TEXT NOT NULL,
                                                 token_symbol    TEXT NOT NULL,
                                                 token_decimals  INT  NOT NULL,
                                                 total_shares    TEXT NOT NULL,
                                                 exchange_rate   TEXT NOT NULL,

                                                 CONSTRAINT pk_operator_strategy PRIMARY KEY (operator_id, strategy_id),
    CONSTRAINT fk_operator_strategy_operator
    FOREIGN KEY (operator_id) REFERENCES operators_snapshot(operator_id)
    ON DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS idx_operator_strategy_operator
    ON operator_strategy (operator_id);
