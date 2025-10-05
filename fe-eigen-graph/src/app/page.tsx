import TokenPageClient from "@/components/page/TokenPageClient";
import {fetchAggregates} from "@/server/operators";

export default async function RootPage() {
    const data = await fetchAggregates();

    const byToken = data.byToken;
    const graph = data.graph;
    const graphDataByToken = Object.fromEntries(
        Object.entries(byToken).map(([k, v]) => [k, v.graph])
    );

    return (
        <TokenPageClient
            byToken={byToken}
            graphDataByToken={graphDataByToken}
            graph={graph}
        />
    );
}
