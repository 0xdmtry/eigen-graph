'use client';
import OperatorsTable from "@/components/operators/OperatorsTable";
import React from "react";
import useSWR from 'swr';
import {notFound, useParams} from 'next/navigation';
import {ApiResponse, GraphItem, TableItem} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import TokenPanel from "@/components/tokens/TokenPanel";
import {baseTokenCards} from "@/data/tokens";
import OperatorAvsDonutChart from "@/components/operators/OperatorAvsDonutChart";
import TokenPrice from "@/components/tokens/TokenPrice";
import {useWebSocket} from "@/hooks/useWebSocket";

const fetcher = (url: string): Promise<ApiResponse> => fetch(url).then((res) => res.json());

const isValidToken = (symbol: string): boolean => {
    if (!symbol) return false;
    const upperCaseSymbol = symbol.toUpperCase();
    return baseTokenCards.some(token => token.symbol.toUpperCase() === upperCaseSymbol);
};

export default function TokenPage() {
    const params = useParams<{ tokenSymbol: string }>();

    if (!isValidToken(params.tokenSymbol)) {
        notFound();
    }

    const seriesData = useWebSocket();

    const series = [
        {
            name: "ETH-USD",
            data: seriesData,
        },
    ];

    const apiUrl = `${process.env.NEXT_PUBLIC_API_URL}/v1/operators/aggregates`;

    const {data, error, isLoading} = useSWR<ApiResponse>(
        apiUrl,
        fetcher
    );

    if (isLoading) return <div>Loading...</div>;
    if (error) return <div>Error fetching data.</div>;
    if (!data) return <div>No data found.</div>;

    const tokensForPanel: Record<string, TableItem[]> = {};
    const graphDataByToken: Record<string, GraphItem[]> = {};
    Object.keys(data.byToken).forEach((token) => {
        tokensForPanel[token] = data.byToken[token].table;
        graphDataByToken[token] = data.byToken[token].graph;
    });

    const selectedTokenSymbol = params.tokenSymbol.toUpperCase();
    const tableDataForSelectedToken = data.byToken[selectedTokenSymbol]?.table || [];
    const barDataForSelectedToken = data.byToken[selectedTokenSymbol]?.bar || [];

    return (
        <div className="space-y-6">
            <main className="max-w-[1400px] mx-auto px-4 sm:px-6 lg:px-8 shadow-md">
                <TokenPanel tokens={tokensForPanel}/>
                <OperatorsTvl barData={barDataForSelectedToken}/>
                <div className="grid grid-cols-10 gap-4">
                    <div className="col-span-4"><OperatorAvsDonutChart tableData={tableDataForSelectedToken}/></div>
                    <div className="col-span-6"><TokenPrice series={series}/></div>
                </div>
                <OperatorsTable
                    tableData={tableDataForSelectedToken}
                />
                <OperatorStrategySankey graphData={data.graph} graphDataByToken={graphDataByToken}/>
            </main>
        </div>
    );
}