"use client";
import React, {useState, useMemo, useEffect} from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";
import {DonutOperatorData, DonutSlice} from "@/types/operators";

const ReactApexChart = dynamic(() => import("react-apexcharts"), {
    ssr: false,
});


export type Donut = Record<string, DonutOperatorData>;

interface OperatorDonutChartProps {
    donutData: Donut;
    topNSlices?: number;
}


const OperatorDonutChart: React.FC<OperatorDonutChartProps> = ({
                                                                   donutData,
                                                                   topNSlices = 7,
                                                               }) => {
    const operatorIds = useMemo(() => Object.keys(donutData), [donutData]);
    const [selectedOperatorId, setSelectedOperatorId] = useState<string | null>(null);

    useEffect(() => {
        if (operatorIds.length > 0 && !selectedOperatorId) {
            let bestOperatorId = operatorIds[0];
            let maxSlices = 0;

            for (const operatorId of operatorIds) {
                const nonZeroSlices = donutData[operatorId].slices.filter(s => s.share > 0).length;
                if (nonZeroSlices > maxSlices) {
                    maxSlices = nonZeroSlices;
                    bestOperatorId = operatorId;
                }
            }
            setSelectedOperatorId(bestOperatorId);
        }
    }, [operatorIds, donutData, selectedOperatorId]);


    const chartData = useMemo(() => {
        if (!selectedOperatorId || !donutData[selectedOperatorId]) {
            return {series: [], labels: [], totalTvl: 0n, originalSlices: []};
        }
        const operator = donutData[selectedOperatorId];
        const totalTvl = operator.slices.reduce((sum, slice) => sum + BigInt(slice.tvlAtomic), 0n);
        const visibleSlices = operator.slices
            .filter(slice => slice.share > 0)
            .sort((a, b) => b.share - a.share);

        let finalSlices: DonutSlice[] = [];
        if (visibleSlices.length > topNSlices) {
            const mainSlices = visibleSlices.slice(0, topNSlices);
            const otherSlices = visibleSlices.slice(topNSlices);
            const otherSum = otherSlices.reduce((sum, s) => sum + s.share, 0);
            const otherTvlSum = otherSlices.reduce((sum, s) => sum + BigInt(s.tvlAtomic), 0n);
            finalSlices = [
                ...mainSlices,
                {share: otherSum, strategyId: "Other", tvlAtomic: otherTvlSum.toString()},
            ];
        } else {
            finalSlices = visibleSlices;
        }

        return {
            series: finalSlices.map(s => s.share * 100),
            labels: finalSlices.map(s => s.strategyId === 'Other' ? 'Other' : s.strategyId),
            totalTvl,
            originalSlices: finalSlices,
        };
    }, [selectedOperatorId, donutData, topNSlices]);

    const options: ApexOptions = { /* ... */};

    return (
        <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="mb-4 flex items-center justify-between">
                <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">
                    Strategy Distribution
                </h3>
                <select
                    value={selectedOperatorId || ''}
                    onChange={(e) => setSelectedOperatorId(e.target.value)}
                    className="rounded border border-gray-300 bg-white px-2 py-1 text-sm dark:border-gray-600 dark:bg-gray-700"
                >
                    {operatorIds.map(id => (
                        <option key={id} value={id}>{id}</option>
                    ))}
                </select>
            </div>
            {chartData.series.length > 0 ? (
                <div className="flex justify-center">
                    <ReactApexChart options={options} series={chartData.series} type="donut" height={350}/>
                </div>
            ) : (
                <div className="flex h-[350px] items-center justify-center">
                    <p className="text-gray-500">{selectedOperatorId ? "This operator has no TVL." : "Select an operator to view."}</p>
                </div>
            )}
        </div>
    );
};

export default OperatorDonutChart;