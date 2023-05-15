-- Add migration script here
alter table user_accounts rename column id TO number;
alter table user_accounts add column id uuid primary key not null;

create unique index user_accounts_idx_number on user_accounts using btree(number);
