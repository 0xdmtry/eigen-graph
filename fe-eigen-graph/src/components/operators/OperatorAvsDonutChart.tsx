"use client";
import React, {useMemo} from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";
import {TableItem} from "@/types/operators";

const ReactApexChart = dynamic(() => import("react-apexcharts"), {
    ssr: false,
    loading: () => <div className="flex h-[350px] items-center justify-center"/>
});


interface OperatorAvsDonutChartProps {
    tableData: TableItem[];
    topN?: number;
}

const shortenId = (id: string, chars = 4): string => {
    if (id.length <= chars * 2 + 2) return id;
    return `${id.substring(0, chars + 2)}...${id.substring(id.length - chars)}`;
};

const OperatorAvsDonutChart: React.FC<OperatorAvsDonutChartProps> = ({tableData, topN = 6}) => {

    console.log("tableData", tableData);
    const chartData = useMemo(() => {
        if (!tableData || tableData.length === 0) return {series: [], labels: []};
        const sortedByAvs = [...tableData].filter(i => i.avsCount > 0).sort((a, b) => b.avsCount - a.avsCount);
        let finalSlices: { label: string; value: number }[] = [];
        if (sortedByAvs.length > topN) {
            const main = sortedByAvs.slice(0, topN);
            const other = sortedByAvs.slice(topN);
            const otherSum = other.reduce((s, op) => s + op.avsCount, 0);
            finalSlices = main.map(op => ({label: shortenId(op.operatorId), value: op.avsCount}));
            if (otherSum > 0) finalSlices.push({label: "Other Operators", value: otherSum});
        } else {
            finalSlices = sortedByAvs.map(op => ({label: shortenId(op.operatorId), value: op.avsCount}));
        }
        return {series: finalSlices.map(s => s.value), labels: finalSlices.map(s => s.label)};
    }, [tableData, topN]);

    console.log("chartData", chartData);


    const options: ApexOptions = {
        chart: {fontFamily: "Outfit, sans-serif", type: "donut", height: 350},
        labels: chartData.labels,
        colors: ["#465fff", "#7592ff", "#027a48", "#f79009", "#d92d20", "#98a2b3", "#344054"],
        plotOptions: {
            pie: {
                donut: {
                    size: "75%",
                    labels: {
                        show: true,
                        name: {show: true, offsetY: -10, formatter: v => v},
                        value: {show: true, offsetY: 10, formatter: v => `${v} AVSs`},
                        total: {show: false}
                    }
                }
            }
        },
        dataLabels: {enabled: false},
        tooltip: {enabled: true, y: {formatter: v => `${v} AVSs`, title: {formatter: n => n}}},
        stroke: {width: 0},
        legend: {show: false},
    };

    const chartKey = useMemo(
        () => `${chartData.labels.join("|")}::${chartData.series.join(",")}`,
        [chartData.labels, chartData.series]
    );

    return (
        <div className="rounded-xl  p-4">
            <h3 className="mb-4 text-lg font-semibold text-gray-800 dark:text-white/90">Operator Distribution by AVS
                Count</h3>
            {chartData.series.length > 0 ? (
                <div id="avs-donut-chart" className="flex justify-center">
                    <ReactApexChart
                        key={chartKey}
                        options={options}
                        series={chartData.series}
                        type="donut"
                        height={350}/>
                </div>
            ) : (
                <div className="flex h-[250px] items-center justify-center">
                    <p className="text-gray-500">No AVS distribution data available.</p>
                </div>
            )}
        </div>
    );
};

export default OperatorAvsDonutChart;
