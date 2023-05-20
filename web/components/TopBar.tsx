import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Container from "@mui/material/Container";

export default function TopBar({ logo }: { logo: React.ReactNode }) {
  return (
    <AppBar color="transparent" elevation={1} position="static">
      <Toolbar disableGutters>
        <Container maxWidth="lg">{logo}</Container>
      </Toolbar>
    </AppBar>
  );
}
