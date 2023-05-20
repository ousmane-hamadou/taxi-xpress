import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import { useState } from "react";

export default function AddTaxiDialog({
  open,
  onClose,
  onSubmit,
}: {
  open: boolean;
  onClose: () => void;
  onSubmit: (data: {
    number: string;
    numOfSeats: number;
    ownerName: string;
  }) => void;
}) {
  const [number, setNumber] = useState("");
  const [ownerName, setOwnerName] = useState("");
  const [numOfSeats, setNumOfSeats] = useState(4);

  function onClick() {
    onSubmit({ number, ownerName, numOfSeats });

    setNumOfSeats(4);
    setOwnerName("");
    setNumber("");
  }

  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>Add new taxi</DialogTitle>

      <Stack
        sx={{ inlineSize: "25ch" }}
        spacing={2}
        p={2}
        component="form"
        autoComplete="off"
      >
        <TextField
          focused
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
        <Button variant="contained" onClick={onClick}>
          Send
        </Button>
      </Stack>
    </Dialog>
  );
}
