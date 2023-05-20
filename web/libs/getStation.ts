export default async function getStation(url: string) {
  const payload = await fetch(url).then((res) => res.json());

  return { name: payload["name"] } as { name: string };
}
