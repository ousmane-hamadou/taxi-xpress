import { API_URL } from "@/libs/constants";

export default async function getStations() {
  const url = `${API_URL}/stations`;

  return (await fetch(url).then((res) => res.json())) as {
    stations: { id: string; name: string }[];
  };
}
