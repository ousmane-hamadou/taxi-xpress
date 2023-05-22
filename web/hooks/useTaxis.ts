import { Taxi } from "@/libs/types";
import useSWR, { Fetcher } from "swr";
import useSearch from "./useSearch";

const fetcher: Fetcher<{ taxis: Taxi[] }, string> = async (url) =>
  fetch(url, { credentials: "include" }).then((res) => {
    if (res.status != 200) {
      throw "taxis not found";
    }

    return res.json();
  });

export default function useTaxis(searchId: string) {
  const { data: d } = useSearch(searchId);

  const { data, isLoading, error } = useSWR(d?._links.taxis.href, fetcher);

  return {
    data,
    isLoading,
    error,
  };
}
