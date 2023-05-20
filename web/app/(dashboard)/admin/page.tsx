"use client";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Container from "@mui/material/Container";

import Layout from "./components/Layout";
import Logo from "@/app/components/Logo";
import PageContent from "./components/PageContent";

function TopBar() {
  return (
    <AppBar elevation={1} position="static" color="transparent">
      <Toolbar disableGutters>
        <Container maxWidth="md">
          <Logo />
        </Container>
      </Toolbar>
    </AppBar>
  );
}

export default function AdminDashboard() {
  return (
    <Layout topBar={<TopBar />}>
      <PageContent />
    </Layout>
  );
}
