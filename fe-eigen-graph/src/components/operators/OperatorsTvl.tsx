"use client";
import React, {useMemo} from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";
import {BarItem} from "@/types/operators";
import {formatPowerOfTen} from "@/utils/number-utils";

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

const OperatorTvlBarChart: React.FC<OperatorTvlBarChartProps> = ({barData, topN = 5}) => {
    const chartData = useMemo(() => {
        if (!barData || barData.length === 0) {
            return {
                isEmpty: true,
                categories: [],
                seriesData: [],
                originalData: [],
            };
        }

        const nonZeroData = barData.filter(item => BigInt(item.tvlTotalAtomic) > BigInt(0));

        const sortedData = [...nonZeroData].sort((a, b) => {
            const valA = BigInt(a.tvlTotalAtomic);
            const valB = BigInt(b.tvlTotalAtomic);
            if (valB > valA) return 1;
            if (valB < valA) return -1;
            return 0;
        });

        const slicedData = sortedData.slice(0, topN);

        if (slicedData.length === 0) {
            return {isEmpty: true, categories: [], seriesData: [], originalData: []};
        }

        return {
            isEmpty: false,
            categories: slicedData.map(item => shortenId(item.operatorId)),
            seriesData: slicedData.map(item => Number(item.tvlTotalAtomic)),
            originalData: slicedData,
        };
    }, [barData, topN]);

    const options: ApexOptions = {
        colors: ["#465fff"],
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
            axisBorder: {show: false},
            axisTicks: {show: false},
        },
        yaxis: {
            logarithmic: true,
            title: {
                text: "TVL × 10⁰⁰  Logarithmic",
                style: {
                    fontWeight: 500
                }
            },
            axisBorder: {show: false},
            labels: {
                formatter: (val: number): string => {
                    if (val <= 0) {
                        return "";
                    }

                    const exponential = val.toExponential(2);
                    const [mantissa, exponent] = exponential.split('e');

                    if (Number(exponent) % 5 === 0) {

                        const superscriptMap: Record<string, string> = {
                            '0': '⁰', '1': '¹', '2': '²', '3': '³', '4': '⁴',
                            '5': '⁵', '6': '⁶', '7': '⁷', '8': '⁸', '9': '⁹',
                            '+': '', '-': '⁻'
                        };

                        const formattedExponent = exponent
                            .split('')
                            .map(char => superscriptMap[char])
                            .join('');

                        return `${mantissa} × 10${formattedExponent}`;
                    }

                    return "";
                }
            }
        },
        fill: {opacity: 1},
        tooltip: {
            custom: function ({series, seriesIndex, dataPointIndex}) {
                const originalItem = chartData.originalData[dataPointIndex];
                if (!originalItem) return '';

                const tvl = formatPowerOfTen(originalItem.tvlTotalAtomic);
                return `
                    <div class="p-2 bg-gray-700 text-white rounded-md shadow-lg text-xs">
                        <div class="font-bold mb-1">Operator: ${originalItem.operatorId}</div>
                        <div><strong>TVL:</strong> ${tvl}</div>
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
                    Top Operators by TVL
                </h3>
            </div>
            <div id="operator-tvl-chart">
                <ReactApexChart options={options} series={series} type="bar" height={350}/>
            </div>
        </div>
    );
};

export default OperatorTvlBarChart;