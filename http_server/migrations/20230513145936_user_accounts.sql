-- Add migration script here
create table user_accounts (
    taxi_number varchar(8),
    password text not null,
    foreign key (taxi_number) references taxis(number)
);
