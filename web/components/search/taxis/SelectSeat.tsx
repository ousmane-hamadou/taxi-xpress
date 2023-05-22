"use client";

import TextField from "@mui/material/TextField";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import { useState } from "react";

export default function SelectSeat() {
  const [num, setNum] = useState(1);
  function onClick() {}

  return (
    <Stack
      component="form"
      direction="column"
      justifyContent="space-around"
      spacing={1}
      sx={{ inlineSize: "10ch" }}
    >
      <TextField
        type="number"
        value={num}
        onChange={(e) => setNum(Number.parseInt(e.target.value))}
        size="small"
        variant="filled"
      />

      <Button onClick={onClick} variant="outlined" size="small">
        Book
      </Button>
    </Stack>
  );
}
