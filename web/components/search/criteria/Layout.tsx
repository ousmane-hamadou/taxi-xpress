import Container from "@mui/material/Container";
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
    <Container maxWidth="xs">
      <Paper variant="outlined">
        <Stack p={2} direction="column">
          {header}
          {departure}
          {arrival}
        </Stack>
      </Paper>
    </Container>
  );
}
