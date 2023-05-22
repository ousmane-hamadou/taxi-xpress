export type Search = {
  id: string;
  criteria: { departure_station: string; arrival_station: string };
  _links: { taxis: { href: string }; self: { href: string } };
};

export type Taxi = {
  number: string;
  model: string;
  departure_time: string;
  available_seats: string;
  journey_id: string;
};
