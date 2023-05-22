"use client";
import Stack from "@mui/material/Stack";
import Box from "@mui/material/Box";
import TextField from "@mui/material/TextField";
import Autocomplete from "@mui/material/Autocomplete";
import Button from "@mui/material/Button";
import { useState } from "react";
import useSWRMutation from "swr/mutation";
import { SEARCHES_API_URL } from "@/libs/constants";
import { useRouter } from "next/navigation";

type Station = { id: string; label: string };

type SearchCriteria = { origin_id: string; destination_id: string };

async function performSearch(url: string, { arg }: { arg: SearchCriteria }) {
  const res = await fetch(url, {
    method: "post",
    headers: new Headers({
      "content-type": "application/json",
    }),
    body: JSON.stringify({
      departure_station: arg.origin_id,
      arrival_station: arg.destination_id,
    }),
    credentials: "include",
  });

  return await res.json();
}

function usePerformSearch(url: string) {
  const { trigger, isMutating } = useSWRMutation(url, performSearch);

  return { trigger, isMutating };
}

export default function SearchForm({ stations }: { stations: Station[] }) {
  const [departure, setDeparture] = useState<Station | null>(null);
  function handleDepartureChange(s: Station | null) {
    setDeparture(s);
  }

  const [arrival, setArrival] = useState<Station | null>(null);
  function handleArrivalChange(s: Station | null) {
    setArrival(s);
  }

  const { trigger, isMutating } = usePerformSearch(SEARCHES_API_URL);

  const router = useRouter();

  async function onSubmit() {
    const data = await trigger({
      origin_id: departure?.id ?? "",
      destination_id: arrival?.id ?? "",
    });

    console.log(data);

    router.push(`/search/${data["id"]}`);
  }

  return (
    <Stack direction="column" spacing={4}>
      <Stack
        component="form"
        direction={{ xs: "column", sm: "row" }}
        spacing={4}
      >
        <Box flexGrow={1}>
          <Autocomplete
            disablePortal
            value={departure}
            onChange={(_, v) => handleDepartureChange(v)}
            options={stations}
            renderInput={({ id, ...params }) => (
              <TextField key={id} {...params} label="Depature" />
            )}
          />
        </Box>

        <Box flexGrow={1}>
          <Autocomplete
            disablePortal
            value={arrival}
            onChange={(_, v) => handleArrivalChange(v)}
            options={stations}
            renderInput={({ id, ...params }) => (
              <TextField key={id} {...params} label="Arrival" />
            )}
          />
        </Box>
      </Stack>

      <Stack justifyContent="center" direction="row">
        <Button
          disabled={isMutating}
          variant="contained"
          size="large"
          onClick={onSubmit}
        >
          Search
        </Button>
      </Stack>
    </Stack>
  );
}
