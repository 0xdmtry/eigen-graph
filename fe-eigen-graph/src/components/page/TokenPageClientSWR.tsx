"use client";
import React, {useMemo, useState} from "react";
import {ApiResponse, GraphItem, TokenSlice} from "@/types/operators";
import {TokenContext} from "@/context/TokenContext";
import {useAggregates} from "@/hooks/useAggregates";
import TokenPanelDnD from "@/components/tokens/TokenPanelDnD";
import OperatorsTvl from "@/components/operators/OperatorsTvl";
import OperatorAvsDonutChart from "@/components/operators/OperatorAvsDonutChart";
import TokenPriceLive from "@/components/tokens/TokenPriceLive";
import OperatorStrategySankey from "@/components/operators/OperatorStrategySankey";
import OperatorsTable from "@/components/operators/OperatorsTable";

type ByToken = Record<string, TokenSlice>;
type GraphByToken = Record<string, GraphItem[]>;

function TopSection({byToken, symbol}: { byToken: ByToken; symbol: string }) {
    const tokensForPanel = useMemo(
        () => Object.fromEntries(Object.entries(byToken).map(([k, v]) => [k, v.table])),
        [byToken]
    );
    const barData = byToken[symbol]?.bar || [];
    const tableData = byToken[symbol]?.table || [];

    return (
        <>
            <div className="min-h-[350px]">
                <TokenPanelDnD tokens={tokensForPanel}/>
            </div>

            <div className="h-[445px] dark:border-gray-800 dark:bg-white/[0.03]">
                <OperatorsTvl barData={barData}/>
            </div>

            <div>
                <div className="grid grid-cols-10 gap-4 h-[385px]">
                    <div className="col-span-4 h-[385px] dark:border-gray-800 dark:bg-white/[0.03]">
                        <OperatorAvsDonutChart tableData={tableData}/>
                    </div>
                    <div className="col-span-6 h-[385px] dark:border-gray-800 dark:bg-white/[0.03]">
                        <TokenPriceLive/>
                    </div>
                </div>
            </div>
        </>
    );
}

function BottomTable({byToken, symbol}: { byToken: ByToken; symbol: string }) {
    const tableData = byToken[symbol]?.table || [];
    return <OperatorsTable tableData={tableData}/>;
}

export default function TokenPageClientSWR({initialData}: { initialData: ApiResponse }) {
    const {data, meta, mutate} = useAggregates({fallbackData: initialData});
    const snapshot = data ?? initialData;

    const [selectedTokenSymbol, _set] = useState("EIGEN");
    const symbol = selectedTokenSymbol || "EIGEN";
    const staleTimeMs = Number(process.env.NEXT_PUBLIC_STALE_TIME_MS ?? "60000");

    const setSelectedTokenSymbol = (s: string) => {
        _set(s);
        if (Date.now() - (meta.fetchedAt || 0) > staleTimeMs) mutate();
    };

    const ctx = useMemo(() => ({selectedTokenSymbol, setSelectedTokenSymbol}), [selectedTokenSymbol]);

    const byToken: ByToken = snapshot.byToken;
    const graph: GraphItem[] = snapshot.graph;
    const graphDataByToken: GraphByToken = useMemo(
        () => Object.fromEntries(Object.entries(byToken).map(([k, v]) => [k, v.graph])),
        [byToken]
    );

    return (
        <div className="space-y-6">
            <main className="max-w-[1400px] mx-auto px-4 sm:px-6 lg:px-8 shadow-md">
                <div className="grid gap-4">
                    <TokenContext.Provider value={ctx}>
                        <TopSection byToken={byToken} symbol={symbol}/>
                    </TokenContext.Provider>

                    <div className="h-[740px] dark:border-gray-800 dark:bg-white/[0.03]">
                        <OperatorStrategySankey graphData={graph} graphDataByToken={graphDataByToken}/>
                    </div>

                    <TokenContext.Provider value={ctx}>
                        <BottomTable byToken={byToken} symbol={symbol}/>
                    </TokenContext.Provider>
                </div>
            </main>
        </div>
    );
}
