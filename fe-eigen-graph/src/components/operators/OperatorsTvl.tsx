"use client";
import React, {useMemo, useState} from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";
import {BarItem} from "@/types/operators";

const ReactApexChart = dynamic(() => import("react-apexcharts"), {
    ssr: false,
});

interface OperatorTvlBarChartProps {
    barData: BarItem[];
    topN?: number;
}

const shortenId = (id: string, chars = 6): string => {
    if (id.length <= chars * 2 + 2) return id;
    return `${id.substring(0, chars + 2)}...${id.substring(id.length - chars)}`;
};

const shouldDefaultToLogScale = (data: BarItem[], skewFactor: number = 10): boolean => {
    if (!data || data.length < 2) {
        return false;
    }

    const values = data.map(item => BigInt(item.tvlTotalAtomic));

    let maxVal = 0n;
    let secondMaxVal = 0n;

    for (const val of values) {
        if (val > maxVal) {
            secondMaxVal = maxVal;
            maxVal = val;
        } else if (val > secondMaxVal) {
            secondMaxVal = val;
        }
    }

    if (secondMaxVal === 0n) {
        return false;
    }

    return maxVal > secondMaxVal * BigInt(skewFactor);
};

const OperatorTvlBarChart: React.FC<OperatorTvlBarChartProps> = ({barData, topN = 10}) => {

    const [isLogScale, setIsLogScale] = useState(() => shouldDefaultToLogScale(barData));


    const chartData = useMemo(() => {
        if (!barData || barData.length === 0) {
            return {
                isEmpty: true,
                categories: [],
                seriesData: [],
                originalData: [],
            };
        }

        const sortedData = [...barData].sort((a, b) => {
            return BigInt(b.tvlTotalAtomic) > BigInt(a.tvlTotalAtomic) ? 1 : -1;
        });

        const slicedData = sortedData.slice(0, topN);

        return {
            isEmpty: false,
            categories: slicedData.map(item => shortenId(item.operatorId)),
            seriesData: slicedData.map(item => Number(BigInt(item.tvlTotalAtomic) / BigInt(10 ** 18))),
            originalData: slicedData,
        };
    }, [barData, topN]);

    const options: ApexOptions = {
        colors: ["#465fff", "#0abde3"],
        chart: {
            fontFamily: "Outfit, sans-serif",
            type: "bar",
            height: 350,
            toolbar: {show: false},
        },

        plotOptions: {
            bar: {
                horizontal: false,
                columnWidth: "55%",
                borderRadius: 4,
            },
        },
        dataLabels: {enabled: false},
        stroke: {show: true, width: 2, colors: ["transparent"]},
        xaxis: {
            categories: chartData.categories,
            labels: {
                style: {
                    fontSize: '12px',
                }
            },
            axisBorder: {
                show: false,
            },
            axisTicks: {
                show: false,
            },
        },
        yaxis: {
            title: {
                text: "TVL (in ETH-equivalent units)",
                style: {
                    fontWeight: 500
                }
            },
            axisBorder: {
                show: false,
            },
            ...(isLogScale && {logarithmic: true}),
        },
        fill: {opacity: 1},
        tooltip: {
            custom: function ({series, seriesIndex, dataPointIndex}) {
                const originalItem = chartData.originalData[dataPointIndex];
                if (!originalItem) return '';

                const tvl = BigInt(originalItem.tvlTotalAtomic).toLocaleString();
                return `
                    <div class="p-2 bg-gray-700 text-white rounded-md shadow-lg">
                        <div class="font-bold text-xs mb-1">Operator: ${originalItem.operatorId}</div>
                        <div><strong>TVL:</strong> ${tvl} (atomic units)</div>
                    </div>
                `;
            },
        },
    };

    const series = [{
        name: "TVL",
        data: chartData.seriesData,
    }];

    if (chartData.isEmpty) {
        return (
            <div
                className="flex h-80 items-center justify-center rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
                <p className="text-gray-500">No data available to display chart.</p>
            </div>
        );
    }

    return (
        <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="mb-4 flex items-center justify-between">
                <h3 className="text-lg font-semibold text-gray-800 dark:text-white/90">
                    Top {topN} Operators by TVL
                </h3>
                <button
                    onClick={() => setIsLogScale(!isLogScale)}
                    className="rounded-md bg-gray-200 px-3 py-1 text-xs font-medium text-gray-800 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                >
                    {isLogScale ? "Use Linear Scale" : "Use Log Scale"}
                </button>
            </div>

            <div id="operator-tvl-chart">
                <ReactApexChart options={options} series={series} type="bar" height={350}/>
            </div>

            {barData.length > topN && (
                <div className="mt-2 text-center text-sm text-gray-500">
                    + {barData.length - topN} more operators
                </div>
            )}
        </div>
    );
};

export default OperatorTvlBarChart;