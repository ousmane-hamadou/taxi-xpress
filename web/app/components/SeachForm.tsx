"use client";

import { API_URL } from "../lib/constants";

async function getStations(url: string) {
  const res = await fetch(url);

  return res.json() as Promise<Stations>;
}

async function getEnpoints() {
  const res = await fetch(API_URL);

  return res.json() as Promise<{ endpoints: { stations: { href: string } } }>;
}
export default async function SearchForm() {
  const { endpoints } = await getEnpoints();

  const { stations } = await getStations(endpoints.stations.href);
  return (
    <form>
      <div className="grid">
        <div>
          <label htmlFor="departure">Departure</label>

          <select name="departure" id="depature">
            {stations.map(({ id, name }) => (
              <option key={id} value={name}>
                {name}
              </option>
            ))}
          </select>
        </div>

        <div>
          <label htmlFor="arrival">Arrival</label>
          <select name="arrival" id="arrival">
            {stations.map(({ id, name }) => (
              <option key={id} value={name}>
                {name}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div className="grid">
        <div>
          <label htmlFor="departure-time">Departure time</label>

          <input type="datetime-local" name="departure-time" />
        </div>
        <div></div>
      </div>
      <div>
        <input type="checkbox" name="roudtrip" id="roundtrip" />
        <label htmlFor="roundtrip">Roundtrip</label>
      </div>
      <div style={{ display: "grid", placeContent: "center" }}>
        <button type="submit">Searches</button>
      </div>
    </form>
  );
}
