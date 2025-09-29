CREATE TABLE IF NOT EXISTS stream_cursors (
                                              token_id   TEXT PRIMARY KEY,
                                              last_ts    BIGINT NOT NULL,
                                              last_id    TEXT   NOT NULL,
                                              updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );

