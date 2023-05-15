-- Add migration script here
drop index index_taxis_av_place;
drop index index_taxis_station;

create index index_taxis_av_place on taxis using hash(available_place);
create index index_taxis_station on taxis using hash(current_station);
