create table if not exists deposits
(
    id          text not null constraint deposits_pk primary key,
    from          text not null,
    to      text not null,
    tx_hash        text not null,
    tx_value int not null,
);

create table total_deposits
(
    id         text not null constraint total_deposits_pk primary key,
    total     int,
);
