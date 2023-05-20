import Container from "@mui/material/Container";
import Box from "@mui/material/Box";

export default function HomeLayout({
  searchForm,
  topBar,
}: {
  searchForm: React.ReactNode;
  topBar: React.ReactNode;
}) {
  return (
    <>
      {topBar}
      <Container maxWidth="lg" component="main">
        <Box pt={3}>{searchForm}</Box>
      </Container>
    </>
  );
}
