'use client';
import OperatorsTable from "@/components/operators/OperatorsTable";
import React from "react";
import useSWR from 'swr';
import {notFound, useParams} from 'next/navigation';
import {ApiResponse, GraphItem, TableItem} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import TokenPanel from "@/components/operators/TokenPanel";
import {baseTokenCards} from "@/data/tokens";
import OperatorAvsDonutChart from "@/components/operators/OperatorAvsDonutChart";

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

    const {data, error, isLoading} = useSWR<ApiResponse>(
        'http://localhost:8000/v1/operators/aggregates',
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

    console.log("data.byToken.ATH.donut", JSON.stringify(data.byToken.ATH.table));

    return (
        <div className="space-y-6">
            <main className="max-w-[1400px] mx-auto px-4 sm:px-6 lg:px-8 shadow-md">
                <TokenPanel tokens={tokensForPanel}/>
                <OperatorsTvl barData={barDataForSelectedToken}/>
                <OperatorAvsDonutChart tableData={tableDataForSelectedToken}/>
                <OperatorsTable
                    tableData={tableDataForSelectedToken}
                />
                <OperatorStrategySankey graphData={data.graph} graphDataByToken={graphDataByToken}/>
            </main>
        </div>
    );
}