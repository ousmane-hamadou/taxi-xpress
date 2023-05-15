drop table if exists user_accounts;
drop table if exists taxis;
drop table if exists stations;

create table stations (
    id uuid primary key,
    name text
);

create table taxis (
    id uuid not null,
    number varchar(7) unique not null,
    max_place integer not null,
    available_place integer not null,
    check(available_place <= max_place),
    current_station uuid references stations,
    destination_station uuid references stations,
    primary key(id, number)
);

create unique index index_taxis_av_place on taxis using btree(available_place);
create unique index index_taxis_station on taxis using btree(current_station);

create table user_accounts (
    taxi_number varchar(8),
    password text not null,
    foreign key (taxi_number) references taxis(number)
);
--create unique index index_ua_taxi_number on taxi_number using btree(available_place);
