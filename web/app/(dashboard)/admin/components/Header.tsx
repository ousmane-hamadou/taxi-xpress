import Container from "@mui/material/Container";
import Stack from "@mui/material/Stack";

export default function Header({ actions }: { actions: React.ReactNode }) {
  return (
    <div style={{ backgroundColor: "#0000000d" }}>
      <Container maxWidth="md">
        <Stack
          paddingY={2}
          spacing={1}
          justifyContent="flex-end"
          direction="row"
        >
          {actions}
        </Stack>
      </Container>
    </div>
  );
}
