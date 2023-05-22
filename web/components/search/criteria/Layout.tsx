import Box from "@mui/material/Box";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";

export default function CriteriaLayout({
  header,
  departure,
  arrival,
}: {
  header: React.ReactNode;
  departure: React.ReactNode;
  arrival: React.ReactNode;
}) {
  return (
    <Box minWidth={250} px={3}>
      <Paper variant="outlined">
        <Stack p={2} direction="column">
          {header}
          {departure}
          {arrival}
        </Stack>
      </Paper>
    </Box>
  );
}
