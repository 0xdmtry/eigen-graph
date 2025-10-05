import TokenPageClientSWR from "@/components/page/TokenPageClientSWR";
import {fetchAggregates} from "@/server/operators";

export default async function RootPage() {
    const data = await fetchAggregates();
    return <TokenPageClientSWR initialData={data}/>;
}