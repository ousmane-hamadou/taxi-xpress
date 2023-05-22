"use client";

import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import Paper from "@mui/material/Paper";
import Divider from "@mui/material/Divider";
import { Taxi } from "@/libs/types";

export default function Taxi({
  choise_seat_num,
  taxi,
}: {
  choise_seat_num: React.ReactNode;
  taxi: Taxi;
}) {
  return (
    <Paper variant="outlined">
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
          <Typography sx={{ opacity: 0.6 }} variant="subtitle2" component="p">
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

        {choise_seat_num}
      </Stack>
    </Paper>
  );
}
