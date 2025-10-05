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
                <div className="grid gap-4">
                    <div className="min-h-[350px]">
                        <TokenPanelDnD tokens={tokensForPanel}/>
                    </div>
                    <div className="h-[445px] dark:border-gray-800 dark:bg-white/[0.03]">
                        <OperatorsTvl barData={barDataForSelectedToken}/>
                    </div>
                    <div>
                        <div className="grid grid-cols-10 gap-4 h-[385px]">
                            <div className="col-span-4 h-[385px] dark:border-gray-800 dark:bg-white/[0.03]">
                                <OperatorAvsDonutChart tableData={tableDataForSelectedToken}/>
                            </div>
                            <div className="col-span-6 h-[385px]  dark:border-gray-800 dark:bg-white/[0.03]">
                                <TokenPriceLive/>
                            </div>
                        </div>
                    </div>
                    <div className="h-[740px] dark:border-gray-800 dark:bg-white/[0.03]">
                        <OperatorStrategySankey graphData={graph} graphDataByToken={graphDataByToken}/>
                    </div>
                    <div>
                        <OperatorsTable tableData={tableDataForSelectedToken}/>
                    </div>
                </div>
            </main>
        </div>
    );
}
