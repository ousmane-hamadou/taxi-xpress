"use client";

import Logo from "@/components/Logo";
import TopBar from "@/components/TopBar";
import Container from "@mui/material/Container";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";

export default function SearchLayout({
  children,
  criteria,
}: {
  children: React.ReactNode;
  criteria: React.ReactNode;
  selection: React.ReactNode;
}) {
  return (
    <>
      <TopBar logo={<Logo />} />
      <Container maxWidth="lg">
        <Grid2 container pt={2}>
          <Grid2 component="section" xs={12} sm="auto">
            {criteria}
          </Grid2>

          <Grid2 component="main" xs={12} sm={6} flexGrow={1}>
            {children}
          </Grid2>
        </Grid2>
      </Container>
    </>
  );
}
