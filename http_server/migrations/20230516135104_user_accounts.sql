-- Add migration script here
create table user_accounts (
    id char(8) primary key references taxis(number),
    full_name varchar(50) not null,
    password text not null
);
