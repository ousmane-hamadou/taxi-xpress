drop view if exists active_taxis_journey;
drop table if exists bookings;
drop table if exists trips;
drop table if exists user_accounts;
drop table if exists taxis;
drop table if exists stations;

create table stations (
    id uuid primary key,
    name text
);

create table taxis (
    id uuid primary key,
    number char(8) unique not null constraint number_len check(char_length(number) = 8),
    number_of_seats integer not null
);

create table user_accounts (
    id char(8) primary key references taxis(number),
    full_name varchar(50) not null,
    password text not null
);

create table trips (
    id uuid primary key,
    taxi_id uuid not null references taxis,
    origin_id uuid not null references stations,
    destination_id uuid not null references stations,
    constraint diff_origin_and_destination check (origin_id != destination_id),
    departure_time timestamp with time zone not null,
    complete boolean not null default false
);

--create index idx_trips_taxi_id on trips using hash(taxi_id);
--create index idx_trips_origin on trips using btree(origin_id, destination_id, complete);

create table bookings (
    id uuid primary key,
    journey_id uuid not null references trips,
    reserved_seats integer not null
);

create index idx_bookings_journey_id on bookings using hash(journey_id);
--
--create view active_taxis_journey as
--    select t.id, count(b.reserved_seats), tp.departure_time from trips tp
--        inner join taxis t on t.id = tp.taxi_id
--        inner join bookings b on b.journey_id = tp.id group by t.id, tp.departure_time;