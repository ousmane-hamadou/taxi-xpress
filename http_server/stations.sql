begin;

insert into stations (id, name) values
(gen_random_uuid()::uuid, 'Grand Marche (Ville)'),
(gen_random_uuid()::uuid, 'Petit Marche (Ville)'),
(gen_random_uuid()::uuid, 'Borongo (Dang)'),
(gen_random_uuid()::uuid, 'Gerite (Bini)');

commit;