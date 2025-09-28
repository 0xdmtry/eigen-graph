'use client';
import OperatorsTable from "@/components/operators/OperatorsTable";
import React from "react";
import useSWR from 'swr';

import type {ApiResponse} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import OperatorDonutChart from "@/components/operators/OperatorDonutChart";

const fetcher = (url: string): Promise<ApiResponse> => fetch(url).then((res) => res.json());

export default function Home() {
    const {data, error, isLoading} = useSWR<ApiResponse>(
        'http://localhost:8000/v1/operators/aggregates',
        fetcher
    );

    if (isLoading) return <div>Loading...</div>;
    if (error) return <div>Error fetching data.</div>;
    if (!data) return <div>No data found.</div>;

    return (
        <div className="space-y-6">
            <OperatorDonutChart donutData={data.donut}/>
            <OperatorStrategySankey graphData={data.graph}/>
            <OperatorsTvl barData={data.bar}/>
            <OperatorsTable
                tableData={data.byToken.EIGEN.table}
            />
        </div>
    );
}
