import useStation from "@/hooks/useStation";
import Station from "./Station";

export default function DeparturelStation({ searchId }: { searchId: string }) {
  const { data } = useStation(searchId, "departure_station");
  return <Station name={data?.name ?? ""} criteria="Departure" />;
}
