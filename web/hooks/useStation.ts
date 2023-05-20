import swrImmatubale from "swr/immutable";
import useSearch from "./useSearch";

const fetcher = async (url: string) =>
  (await fetch(url, { credentials: "include" }).then((res) => res.json())) as {
    name: string;
  };

export default function useStation(
  searchId: string,
  criteria: "departure_station" | "arrival_station"
) {
  const { data: d } = useSearch(searchId);
  const { data, isLoading, error } = swrImmatubale(
    d?.criteria[criteria],
    fetcher
  );

  return {
    data,
    isLoading,
    isError: error,
  };
}
