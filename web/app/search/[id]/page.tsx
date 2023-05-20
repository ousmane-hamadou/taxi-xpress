"use client";

import Container from "@mui/material/Container";

export default function Searches({
  params: { id },
}: {
  params: { id: string };
}) {
  return (
    <Container maxWidth="md" component="main">
      {id}
    </Container>
  );
}
