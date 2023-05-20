"use client";

import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import { useRouter } from "next/navigation";

export default function Header() {
  const router = useRouter();
  async function goHome() {
    await router.push("/");
  }

  return (
    <Stack
      component="header"
      direction="row"
      justifyContent="space-between"
      alignItems="baseline"
    >
      <Typography component="p" variant="subtitle1">
        Search for
      </Typography>

      <Button size="small" onClick={goHome} variant="text">
        Modify
      </Button>
    </Stack>
  );
}
