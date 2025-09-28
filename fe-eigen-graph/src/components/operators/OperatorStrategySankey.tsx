"use client";
import React, {useMemo} from 'react';
import {Chart} from "react-google-charts";

export interface GraphItem {
    operatorId: string;
    strategyId: string;
    weightAtomic: string;
}

interface OperatorStrategySankeyProps {
    graphData: GraphItem[];
    weightThreshold?: number;
}

const shortenId = (id: string, chars = 6): string => {
    if (id.length <= chars * 2 + 2) return id;
    return `${id.substring(0, chars + 2)}...${id.substring(id.length - chars)}`;
};

const OperatorStrategySankey: React.FC<OperatorStrategySankeyProps> = ({
                                                                           graphData,
                                                                           weightThreshold = 0.01
                                                                       }) => {

    const sankeyData = useMemo(() => {
        if (!graphData) return [["From", "To", "Weight"]];

        const chartData: (string | number)[][] = [["From", "To", "Weight"]];

        const operatorTotals = new Map<string, bigint>();
        graphData.forEach(item => {
            const total = operatorTotals.get(item.operatorId) || 0n;
            operatorTotals.set(item.operatorId, total + BigInt(item.weightAtomic));
        });

        for (const item of graphData) {
            const totalWeight = operatorTotals.get(item.operatorId);
            if (!totalWeight || totalWeight === 0n) continue;

            const normalizedWeight = Number(BigInt(item.weightAtomic) * 10000n / totalWeight) / 10000;

            if (normalizedWeight < weightThreshold) continue;

            chartData.push([
                shortenId(item.operatorId),
                shortenId(item.strategyId),
                normalizedWeight * 100,
            ]);
        }

        return chartData;
    }, [graphData, weightThreshold]);

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

    if (sankeyData.length <= 1) {
        return (
            <div
                className="flex h-96 items-center justify-center rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
                <p className="text-gray-500">No significant operator allocations to display.</p>
            </div>
        );
    }

    return (
        <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <h3 className="mb-4 text-lg font-semibold text-gray-800 dark:text-white/90">
                Operator → Strategy Allocation
            </h3>
            <Chart
                chartType="Sankey"
                width="100%"
                height="500px"
                data={sankeyData}
                options={options}
                loader={<div>Loading Chart...</div>}
            />
        </div>
    );
};

export default OperatorStrategySankey;