DROP TABLE IF EXISTS ticks;

CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS ticks (
                                     product_id  TEXT         NOT NULL,
                                     time        TIMESTAMPTZ  NOT NULL,
                                     id          BIGSERIAL    NOT NULL,
                                     price       NUMERIC      NOT NULL,
                                     PRIMARY KEY (product_id, time, id)
    );

SELECT create_hypertable('ticks', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_ticks_time_desc ON ticks (time DESC);

CREATE INDEX IF NOT EXISTS idx_ticks_product_id ON ticks (product_id);
