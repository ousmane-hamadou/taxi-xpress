"use client";

import ArrivalStation from "@/components/search/criteria/ArrivalStation";
import DeparturelStation from "@/components/search/criteria/DepartureStation";
import Header from "@/components/search/criteria/Header";
import CriteriaLayout from "@/components/search/criteria/Layout";

export default function Criteria({
  params: { id },
}: {
  params: { id: string };
}) {
  return (
    <>
      <CriteriaLayout
        header={<Header />}
        departure={<DeparturelStation searchId={id} />}
        arrival={<ArrivalStation searchId={id} />}
      />
    </>
  );
}
