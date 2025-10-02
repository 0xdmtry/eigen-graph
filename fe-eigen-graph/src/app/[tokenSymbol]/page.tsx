import {notFound} from "next/navigation";
import {baseTokenCards} from "@/data/tokens";
import TokenPageClient from "@/components/page/TokenPageClient";
import {ApiResponse, GraphItem, TableItem} from "@/types/operators";

export default async function Page({params}: { params: { tokenSymbol: string } }) {
    const symbol = params.tokenSymbol?.toUpperCase();
    const isValid = !!symbol && baseTokenCards.some(t => t.symbol.toUpperCase() === symbol);
    if (!isValid) notFound();

    const base = process.env.NEXT_PUBLIC_API_URL;
    if (!base) throw new Error("API URL is not configured");
    const res = await fetch(`${base}/v1/operators/aggregates`, {cache: "no-store"});
    if (!res.ok) throw new Error("Failed to fetch operators");
    const data: ApiResponse = await res.json();

    const tokensForPanel: Record<string, TableItem[]> = {};
    const graphDataByToken: Record<string, GraphItem[]> = {};
    Object.keys(data.byToken).forEach(token => {
        tokensForPanel[token] = data.byToken[token].table;
        graphDataByToken[token] = data.byToken[token].graph;
    });

    const tableDataForSelectedToken = data.byToken[symbol!]?.table || [];
    const barDataForSelectedToken = data.byToken[symbol!]?.bar || [];

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
