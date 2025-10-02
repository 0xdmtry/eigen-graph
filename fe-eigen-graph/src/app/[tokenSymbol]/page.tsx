import {notFound} from "next/navigation";
import {GraphItem, TableItem} from "@/types/operators";
import {fetchAggregates} from "@/server/operators";
import TokenPageClient from "@/components/page/TokenPageClient";
import {isValidToken, normalizeToken} from "@/modules/tokens";

export default async function Page({params}: { params: Promise<{ tokenSymbol: string }> }) {
    const {tokenSymbol} = await params;
    if (!isValidToken(tokenSymbol)) notFound();
    const symbol = normalizeToken(tokenSymbol);

    const data = await fetchAggregates();

    const tokensForPanel: Record<string, TableItem[]> = {};
    const graphDataByToken: Record<string, GraphItem[]> = {};
    Object.keys(data.byToken).forEach(token => {
        tokensForPanel[token] = data.byToken[token].table;
        graphDataByToken[token] = data.byToken[token].graph;
    });

    const tableDataForSelectedToken = data.byToken[symbol]?.table || [];
    const barDataForSelectedToken = data.byToken[symbol]?.bar || [];

    return (
        <TokenPageClient
            tokensForPanel={tokensForPanel}
            graphDataByToken={graphDataByToken}
            tableDataForSelectedToken={tableDataForSelectedToken}
            barDataForSelectedToken={barDataForSelectedToken}
            graph={data.graph}
        />
    );
}
