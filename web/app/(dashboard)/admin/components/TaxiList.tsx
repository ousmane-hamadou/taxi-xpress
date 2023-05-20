import Typography from "@mui/material/Typography";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import { Box } from "@mui/material";

function Taxi({
  taxi,
}: {
  taxi: { number: string; owner: string; numOfSeats: number };
}) {
  return (
    <Paper>
      <Stack
        sx={{ "&:hover": { bgcolor: "#0000000d" } }}
        direction="column"
        spacing={1}
        p={3}
        mb={2}
      >
        <Typography variant="h5">{taxi.owner}</Typography>
        <Stack direction="column">
          <Typography variant="subtitle1">
            Registration Number: {taxi.number}
          </Typography>
          <Typography variant="subtitle2">
            Number of seats: {taxi.numOfSeats}
          </Typography>
        </Stack>
      </Stack>
    </Paper>
  );
}

export default function TaxiList() {
  return (
    <Box p={2}>
      <Taxi
        taxi={{ number: "kjjdksd", owner: "Oumar Moussa", numOfSeats: 6 }}
      />
      <Taxi taxi={{ number: "kjjdksd", owner: "Djido Ali", numOfSeats: 4 }} />
    </Box>
  );
}
