import useStation from "@/hooks/useStation";
import Station from "./Station";

export default function ArrivalStation({ searchId }: { searchId: string }) {
  const { data } = useStation(searchId, "arrival_station");
  return <Station name={data?.name || "Unk"} criteria="Arrival" />;
}
