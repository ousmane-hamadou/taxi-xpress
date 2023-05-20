"use client";

import { ThemeProvider, createTheme } from "@mui/material";
import { SWRConfig } from "swr";

const theme = createTheme();

export default function Provider({ children }: { children: React.ReactNode }) {
  return (
    <ThemeProvider theme={theme}>
      <SWRConfig>{children}</SWRConfig>
    </ThemeProvider>
  );
}
