import { SEARCHES_API_URL } from "@/libs/constants";
import useSWRImmutable from "swr/immutable";
import type { Search } from "@/libs/types";
import { Fetcher } from "swr";

const fetcher: Fetcher<Search, string> = async (id: string) =>
  fetch(`${SEARCHES_API_URL}/${id}`, { credentials: "include" }).then((res) =>
    res.json()
  );

export default function useSearch(searchId: string) {
  const { data, isLoading, error } = useSWRImmutable(searchId, fetcher);

  return {
    data,
    isLoading,
    error,
  };
}
