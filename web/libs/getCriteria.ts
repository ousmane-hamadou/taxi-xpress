import getSearch from "./getSearch";

export default async function getCriteria(searchId: string, cookie: string) {
  const payload = await getSearch(searchId, cookie);
  return payload["criteria"] as {
    departure_station: string;
    arrival_station: string;
  };
}
