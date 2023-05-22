"use client";

import Stack from "@mui/material/Stack";
import SelectSeat from "./SelectSeat";
import Taxi from "./Taxi";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import CircularProgress from "@mui/material/CircularProgress";
import Paper from "@mui/material/Paper";
import Divider from "@mui/material/Divider";

import useTaxis from "@/hooks/useTaxis";

export default function TaxiList({ searchId }: { searchId: string }) {
  const { data, isLoading, error } = useTaxis(searchId);

  // useEffect(() => console.log(error));

  if (isLoading) {
    return (
      <Box sx={{ display: "grid", placeContent: "center" }}>
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box sx={{ display: "grid", placeContent: "center" }}>
        <Typography variant="h5" component="p">
          No availlable taxis for this way
        </Typography>
      </Box>
    );
  }

  return (
    <Stack spacing={2}>
      {data?.taxis.map((taxi) => (
        <Paper key={taxi.journey_id} variant="outlined">
          <Stack
            sx={{ blockSize: 100 }}
            direction={{ sm: "column", md: "row" }}
            p={2}
            spacing={1}
            justifyContent="space-between"
          >
            <Stack direction="column">
              <Typography variant="subtitle1" component="p">
                {taxi.model}
              </Typography>
              <Typography
                sx={{ opacity: 0.6 }}
                variant="subtitle2"
                component="p"
              >
                {taxi.number}
              </Typography>
            </Stack>

            <Stack direction="column" px={2}>
              <Typography
                textAlign="center"
                variant="subtitle1"
                sx={{ opacity: 0.7 }}
                component="p"
              >
                Depart dans
              </Typography>
              <Paper variant="outlined">
                <Stack
                  spacing={1}
                  justifyContent="center"
                  direction="row"
                  px={1}
                  py={0.1}
                >
                  <Typography variant="subtitle1" component="p">
                    {taxi.departure_time.substring(0, 8)}
                  </Typography>
                  <Divider orientation="vertical"></Divider>
                  <Typography variant="subtitle1" component="p">
                    {taxi.available_seats} place disponible
                  </Typography>
                </Stack>
              </Paper>
            </Stack>
            <SelectSeat />
          </Stack>
        </Paper>
      ))}
    </Stack>
  );
}
