"use client";
import React, {useMemo} from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";
import {TableItem} from "@/types/operators";

const ReactApexChart = dynamic(() => import("react-apexcharts"), {
    ssr: false,
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
    const chartData = useMemo(() => {
        if (!tableData || tableData.length === 0) {
            return {series: [], labels: []};
        }

        const sortedByAvs = [...tableData]
            .filter(item => item.avsCount > 0)
            .sort((a, b) => b.avsCount - a.avsCount);

        let finalSlices: { label: string; value: number }[] = [];

        if (sortedByAvs.length > topN) {
            const mainOperators = sortedByAvs.slice(0, topN);
            const otherOperators = sortedByAvs.slice(topN);
            const otherAvsSum = otherOperators.reduce((sum, op) => sum + op.avsCount, 0);

            finalSlices = mainOperators.map(op => ({
                label: shortenId(op.operatorId),
                value: op.avsCount,
            }));
            if (otherAvsSum > 0) {
                finalSlices.push({label: "Other Operators", value: otherAvsSum});
            }
        } else {
            finalSlices = sortedByAvs.map(op => ({
                label: shortenId(op.operatorId),
                value: op.avsCount,
            }));
        }

        return {
            series: finalSlices.map(s => s.value),
            labels: finalSlices.map(s => s.label),
        };
    }, [tableData, topN]);

    const options: ApexOptions = {
        chart: {
            fontFamily: "Outfit, sans-serif",
            type: "donut",
            height: 350,
        },
        labels: chartData.labels,
        colors: ["#465fff", "#7592ff", "#027a48", "#f79009", "#d92d20", "#98a2b3", "#344054"],
        plotOptions: {
            pie: {
                donut: {
                    size: "75%",
                    labels: {
                        show: true,
                        name: {
                            show: true,
                            offsetY: -10,
                            formatter: (val) => val,
                        },
                        value: {
                            show: true,
                            offsetY: 10,
                            formatter: (val) => `${val} AVSs`,
                        },
                        total: {
                            show: false,
                        }
                    },
                },
            },
        },
        dataLabels: {enabled: false},
        tooltip: {
            enabled: true,
            y: {
                formatter: (val) => `${val} AVSs`,
                title: {
                    formatter: (seriesName) => seriesName,
                },
            },
        },
        stroke: {width: 0},
        legend: {show: false},
        responsive: [
            {
                breakpoint: 480,
                options: {
                    chart: {
                        width: "100%",
                    },
                },
            },
        ],
    };

    return (
        <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <h3 className="mb-4 text-lg font-semibold text-gray-800 dark:text-white/90">
                Operator Distribution by AVS Count
            </h3>
            {chartData.series.length > 0 ? (
                <div id="avs-donut-chart" className="flex justify-center">
                    <ReactApexChart options={options} series={chartData.series} type="donut" height={350}/>
                </div>
            ) : (
                <div className="flex h-[350px] items-center justify-center">
                    <p className="text-gray-500">No AVS distribution data available.</p>
                </div>
            )}
        </div>
    );
};

export default OperatorAvsDonutChart;