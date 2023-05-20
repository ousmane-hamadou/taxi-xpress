-- Add migration script here
create table taxis (
    id uuid primary key,
    number char(8) unique not null constraint number_len check(char_length(number) = 8),
    number_of_seats integer not null
);
