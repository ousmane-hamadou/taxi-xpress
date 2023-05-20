import Typography from "@mui/material/Typography";

export default function Logo() {
  return (
    <Typography
      variant="h4"
      noWrap
      component="a"
      href="/"
      sx={{
        mr: 2,
        color: "warning.main",
        fontWeight: 700,
        letterSpacing: "0",
        textDecoration: "none",
      }}
    >
      TaxiXpress
    </Typography>
  );
}
