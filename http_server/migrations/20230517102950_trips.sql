-- Add migration script here
create table trips (
    id uuid primary key,
    taxi_id uuid not null references taxis,
    origin_id uuid not null references stations,
    destination_id uuid not null references stations,
    constraint diff_origin_and_destination check (origin_id != destination_id),
    departure_time timestamp with time zone not null,
    complete boolean not null default false
);
