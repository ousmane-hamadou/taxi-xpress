import Container from "@mui/material/Container";
import Button from "@mui/material/Button";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";

import { useState } from "react";
import { Box } from "@mui/material";
import useAddNewTaxi from "@/app/lib/hooks/useAddNewTaxi";
import { API_URL } from "@/libs/constants";

function AddTaxiForm() {
  const [number, setNumber] = useState("");
  const [ownerName, setOwnerName] = useState("");
  const [numOfSeats, setNumOfSeats] = useState(4);

  const { trigger } = useAddNewTaxi(`${API_URL}/admin/create-taxi`);

  async function onSubmit() {
    await trigger({
      num_of_seats: numOfSeats,
      owner: ownerName,
      registration_num: number,
    });
  }
  return (
    <Stack spacing={2} component="form" autoComplete="off">
      <TextField
        type="text"
        label="Taxi registration number"
        required
        size="small"
        value={number}
        onChange={(e) => {
          setNumber(e.target.value);
        }}
      />
      <TextField
        type="number"
        label="Number of seats"
        required
        size="small"
        value={numOfSeats}
        onChange={(e) => setNumOfSeats(Number.parseInt(e.target.value))}
      />
      <TextField
        type="text"
        label="Taxi owner fullname"
        required
        value={ownerName}
        onChange={(e) => {
          setOwnerName(e.target.value);
        }}
        size="small"
      />
      <Button variant="contained" onClick={async () => await onSubmit()}>
        Send
      </Button>
    </Stack>
  );
}

export default function PageContent() {
  return (
    <>
      <Container maxWidth="sm">
        <Box pt={8}>
          <AddTaxiForm />
        </Box>
      </Container>
    </>
  );
}
