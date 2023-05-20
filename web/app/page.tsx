"use client";

import Logo from "@/components/Logo";
import FormHeader from "@/components/home/FormHeader";
import HomeLayout from "@/components/home/Layout";
import Search from "@/components/home/Search";
import SearchForm from "@/components/home/SearchForm";
import TopBar from "@/components/TopBar";
import { STATIONS_API_URL } from "@/libs/constants";

export function prefetchStations() {
  void listStation();
}

async function listStation() {
  const res = await fetch(STATIONS_API_URL);

  return (await res.json())["stations"] as { id: string; name: string }[];
}

export default async function Home() {
  const stations = await listStation();
  const options = stations.map(({ id, name }) => ({ id, label: name }));

  return (
    <HomeLayout
      topBar={<TopBar logo={<Logo />} />}
      searchForm={
        <Search
          header={<FormHeader />}
          sForm={<SearchForm stations={options} />}
        />
      }
    />
  );
}
