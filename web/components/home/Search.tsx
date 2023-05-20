import Stack from "@mui/material/Stack";
import Paper from "@mui/material/Paper";
import Divider from "@mui/material/Divider";

export default function Search({
  header,
  sForm,
}: {
  header: React.ReactNode;
  sForm: React.ReactNode;
}) {
  return (
    <Paper variant="outlined">
      <Stack direction="column" spacing={2} p={4}>
        <Stack spacing={1}>
          {header}
          <Divider></Divider>
        </Stack>
        {sForm}
      </Stack>
    </Paper>
  );
}
