CREATE TABLE IF NOT EXISTS deposits (
    id TEXT NOT NULL CONSTRAINT deposits_pk PRIMARY KEY,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    tx_hash TEXT NOT NULL,
    tx_value TEXT NOT NULL
);


CREATE TABLE IF NOT EXISTS total_deposits (
    total_value TEXT NOT NULL CONSTRAINT deposits_pk PRIMARY KEY,
);
