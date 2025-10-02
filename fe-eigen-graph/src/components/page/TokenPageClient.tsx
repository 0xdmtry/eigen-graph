"use client";
import OperatorsTable from "@/components/operators/OperatorsTable";
import React from "react";
import {GraphItem, TableItem, BarItem} from "@/types/operators";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import TokenPanel from "@/components/tokens/TokenPanel";
import OperatorAvsDonutChart from "@/components/operators/OperatorAvsDonutChart";
import TokenPriceLive from "@/components/tokens/TokenPriceLive";

export default function TokenPageClient({
                                            tokensForPanel,
                                            graphDataByToken,
                                            tableDataForSelectedToken,
                                            barDataForSelectedToken,
                                            graph,
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
                <TokenPanel tokens={tokensForPanel}/>
                <OperatorsTvl barData={barDataForSelectedToken}/>
                <div className="grid grid-cols-10 gap-4">
                    <div className="col-span-4">
                        <OperatorAvsDonutChart tableData={tableDataForSelectedToken}/>
                    </div>
                    <div className="col-span-6">
                        <TokenPriceLive/>
                    </div>
                </div>
                <OperatorsTable tableData={tableDataForSelectedToken}/>
                <OperatorStrategySankey graphData={graph} graphDataByToken={graphDataByToken}/>
            </main>
        </div>
    );
}
