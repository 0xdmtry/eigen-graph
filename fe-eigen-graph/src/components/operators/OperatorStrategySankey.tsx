"use client";
import React, {useMemo, useState} from 'react';
import {Chart} from "react-google-charts";
import {baseTokenCards} from "@/data/tokens";
import Image from "next/image";
import Badge from "@/components/ui/badge/Badge";


export interface GraphItem {
    operatorId: string;
    strategyId: string;
    weightAtomic: string;
}

interface OperatorStrategySankeyProps {
    graphData: GraphItem[];
    graphDataByToken: Record<string, GraphItem[]>;
    weightThreshold?: number;
}

const shortenId = (id: string, chars = 6): string => {
    if (id.length <= chars * 2 + 2) return id;
    return `${id.substring(0, chars + 2)}...${id.substring(id.length - chars)}`;
};

const OperatorStrategySankey: React.FC<OperatorStrategySankeyProps> = ({
                                                                           graphData,
                                                                           graphDataByToken,
                                                                           weightThreshold = 0.01
                                                                       }) => {
    const [selectedToken, setSelectedToken] = useState<string | null>(null);

    const handleBadgeClick = (symbol: string) => {
        setSelectedToken(prev => (prev === symbol ? null : symbol));
    };

    const sankeyData = useMemo(() => {
        const activeData = selectedToken ? graphDataByToken[selectedToken] : graphData;

        if (!activeData || activeData.length === 0) return [["From", "To", "Weight"]];

        const chartData: (string | number)[][] = [["From", "To", "Weight"]];
        const operatorTotals = new Map<string, bigint>();

        activeData.forEach(item => {
            const total = operatorTotals.get(item.operatorId) || BigInt(0);
            operatorTotals.set(item.operatorId, total + BigInt(item.weightAtomic));
        });

        for (const item of activeData) {
            const totalWeight = operatorTotals.get(item.operatorId);
            if (!totalWeight || totalWeight === BigInt(0)) continue;

            const normalizedWeight = Number(BigInt(item.weightAtomic) * BigInt(10000) / totalWeight) / 10000;

            if (normalizedWeight < weightThreshold) continue;

            chartData.push([
                shortenId(item.operatorId),
                shortenId(item.strategyId),
                normalizedWeight * 100,
            ]);
        }

        return chartData;
    }, [graphData, graphDataByToken, selectedToken, weightThreshold]);

    const options = {
        sankey: {
            node: {
                colors: ["#465fff", "#0abde3", "#a29bfe", "#747d8c"],
                label: {
                    fontName: 'Outfit',
                    fontSize: 12,
                    color: '#888',
                    bold: false,
                },
            },
            link: {
                colorMode: 'gradient',
                colors: ["#465fff", "#0abde3", "#a29bfe", "#747d8c"]
            },
        },
        backgroundColor: 'transparent',
    };

    return (
        <div className="rounded-xl   p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <h3 className="mb-4 text-lg font-semibold text-gray-800 dark:text-white/90">
                Operator → Strategy Allocation: {(selectedToken) ? selectedToken : "All Tokens"}
            </h3>

            {
                (sankeyData.length <= 1) ? (
                    <div className="flex items-center justify-center h-[500px]">
                        <p className="text-gray-500">No significant operator allocations to display.</p>
                    </div>
                ) : (

                    <div className="h-[500px]"><Chart
                        chartType="Sankey"
                        width="100%"
                        height="500px"
                        data={sankeyData}
                        options={options}
                        loader={<div>Loading Chart...</div>}
                    /></div>)
            }

            <div className="mt-4 flex flex-wrap items-center gap-2 pt-4 dark:border-gray-800">
                {baseTokenCards.map((item, i) => (
                    <button key={item.symbol ?? i} onClick={() => handleBadgeClick(item.symbol)}
                            className={`flex-none rounded-full ${(item.symbol === selectedToken) ? 'opacity-50' : ''}`}>
                        <Badge variant="solid">
                            <div
                                className="flex h-[20px] w-[20px] flex-shrink-0 items-center justify-center rounded-xl">
                                {item.icon ? (<Image
                                    src={`/images/tokens/${item.icon}.png`}
                                    alt={item.name}
                                    width={14}
                                    height={14}
                                />) : (<span
                                    className="text-xs font-bold text-gray-500 dark:text-gray-400">{item.symbol.charAt(0).toUpperCase()}</span>)}
                            </div>
                            {item.symbol}
                        </Badge>
                    </button>
                ))}
            </div>
        </div>
    );
};

export default OperatorStrategySankey;