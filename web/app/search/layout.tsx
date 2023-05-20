"use client";

import Logo from "@/components/Logo";
import TopBar from "@/components/TopBar";
import { Container } from "@mui/material";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";

export default function SearchLayout({
  children,
  criteria,
  selection,
}: {
  children: React.ReactNode;
  criteria: React.ReactNode;
  selection: React.ReactNode;
}) {
  return (
    <>
      <TopBar logo={<Logo />} />
      <Container maxWidth="md">
        <Grid2 container spacing={4} pt={2}>
          <Grid2 component="section" xs={12} sm="auto">
            {criteria}
          </Grid2>

          <Grid2 component="main" xs={12} sm={6}>
            {children}
          </Grid2>

          <Grid2 component="section" xs={12} sm="auto">
            {selection}
          </Grid2>
        </Grid2>
      </Container>
    </>
  );
}
