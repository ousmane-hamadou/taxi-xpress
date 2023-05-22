"use client";

import TaxiList from "@/components/search/taxis/TaxiList";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";

export default function Searches({
  params: { id },
}: {
  params: { id: string };
}) {
  return (
    <Stack spacing={2}>
      <Typography sx={{ opacity: 0.6 }} variant="h6" component="p">
        Choise your taxi
      </Typography>
      <TaxiList searchId={id} />
    </Stack>
  );
}
