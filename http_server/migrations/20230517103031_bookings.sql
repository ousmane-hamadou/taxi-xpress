-- Add migration script here
create table bookings (
    id uuid primary key,
    journey_id uuid not null references trips,
    reserved_seats integer not null
);

create index idx_bookings_journey_id on bookings using hash(journey_id);
