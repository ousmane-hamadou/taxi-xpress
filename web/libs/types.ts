export type Search = {
  id: string;
  criteria: { departure_station: string; arrival_station: string };
  _links: { taxis: string; self: string };
};
