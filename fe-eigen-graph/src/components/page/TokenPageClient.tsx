"use client";
import React, {Suspense} from "react";
import {GraphItem, TableItem, BarItem} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import TokenPanelDnD from "@/components/tokens/TokenPanelDnD";
import OperatorAvsDonutChart from "@/components/operators/OperatorAvsDonutChart";
import TokenPriceLive from "@/components/tokens/TokenPriceLive";
import OperatorsTable from "@/components/operators/OperatorsTable";

export default function TokenPageClient({
                                            tokensForPanel,
                                            graphDataByToken,
                                            tableDataForSelectedToken,
                                            barDataForSelectedToken,
                                            graph
                                        }: {
    tokensForPanel: Record<string, TableItem[]>;
    graphDataByToken: Record<string, GraphItem[]>;
    tableDataForSelectedToken: TableItem[];
    barDataForSelectedToken: BarItem[];
    graph: GraphItem[];
}) {
    return (
        <div className="space-y-6">
            <main className="max-w-[1400px] mx-auto px-4 sm:px-6 lg:px-8 shadow-md">
                <TokenPanelDnD tokens={tokensForPanel}/>
                <Suspense fallback={<div
                    className="rounded-xl border border-gray-200 bg-white p-4 h-80 dark:border-gray-800 dark:bg-white/[0.03]"/>}>
                    <OperatorsTvl barData={barDataForSelectedToken}/>
                </Suspense>
                <div className="grid grid-cols-10 gap-4">
                    <div className="col-span-4">
                        <Suspense fallback={<div
                            className="rounded-xl border border-gray-200 bg-white p-4 h-[350px] dark:border-gray-800 dark:bg-white/[0.03]"/>}>
                            <OperatorAvsDonutChart tableData={tableDataForSelectedToken}/>
                        </Suspense>
                    </div>
                    <div className="col-span-6">
                        <Suspense fallback={<div
                            className="rounded-xl border border-gray-200 bg-white p-4 h-[335px] dark:border-gray-800 dark:bg-white/[0.03]"/>}>
                            <TokenPriceLive/>
                        </Suspense>
                    </div>
                </div>
                <OperatorsTable tableData={tableDataForSelectedToken}/>
                <OperatorStrategySankey graphData={graph} graphDataByToken={graphDataByToken}/>
            </main>
        </div>
    );
}
