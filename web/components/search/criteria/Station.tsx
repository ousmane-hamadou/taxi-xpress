import Stack from "@mui/material/Stack";
import LocationOn from "@mui/icons-material/LocationOn";
import Typography from "@mui/material/Typography";

export default function Station({
  name,
  criteria,
}: {
  name: string;
  criteria: string;
}) {
  return (
    <Stack
      direction="row"
      spacing={2}
      alignItems="flex-start"
      paddingY={3}
      paddingX={2}
    >
      <LocationOn />

      <Stack direction="column" spacing={1}>
        <Typography variant="body1" component="p" sx={{ opacity: 1 }}>
          {criteria}
        </Typography>

        <Typography variant="body1" component="p" sx={{ opacity: 0.6 }}>
          {name}
        </Typography>
      </Stack>
    </Stack>
  );
}
