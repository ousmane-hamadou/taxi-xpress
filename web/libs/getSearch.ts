import { SEARCHES_API_URL } from "@/libs/constants";
import { cookies } from "next/headers";

type Search = {
  id: string;
  criteria: { departure_station: string; arrival_station: string };
  _links: { taxis: string; self: string };
};

export default function getSearch(id: string, cookie: string) {
  return fetch(`${SEARCHES_API_URL}/${id}`, {
    headers: new Headers({ Cookie: `${id}=${cookie}` }),
  }).then((res) => res.json());
}
