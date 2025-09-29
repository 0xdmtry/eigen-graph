'use client';
import OperatorsTable from "@/components/operators/OperatorsTable";
import React from "react";
import useSWR from 'swr';
import {notFound, useParams} from 'next/navigation';
import type {ApiResponse, TableItem} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import OperatorDonutChart from "@/components/operators/OperatorDonutChart";
import TokenPanel from "@/components/operators/TokenPanel";
import TokenAutocomplete from "@/components/operators/TokenAutocomplete";
import {baseTokenCards} from "@/data/tokens";

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

    let tokens: Record<string, TableItem[]> = {};

    Object.keys(data.byToken).forEach((token) => {
        tokens[token] = data.byToken[token].table;
    })

    return (
        <div className="space-y-6">
            <main className="max-w-[1400px] mx-auto px-4 sm:px-6 lg:px-8  shadow-md">
                <TokenPanel tokens={tokens}/>
            </main>
            {/*<OperatorDonutChart donutData={data.donut}/>*/}
            {/*<OperatorStrategySankey graphData={data.graph}/>*/}
            {/*<OperatorsTvl barData={data.bar}/>*/}
            {/*<OperatorsTable*/}
            {/* tableData={data.byToken.EIGEN.table}*/}
            {/*/>*/}
        </div>
    );
}