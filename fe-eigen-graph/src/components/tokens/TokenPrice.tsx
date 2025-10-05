"use client";
import React from "react";
import {ApexOptions} from "apexcharts";
import dynamic from "next/dynamic";

const ReactApexChart = dynamic(() => import("react-apexcharts"), {
    ssr: false,
    loading: () => <div className="h-[335px]"/>
});

interface LineChartProps {
    series: { name: string; data: [number, number][] }[];
}

export default function TokenPrice({series}: LineChartProps) {
    const empty = !series?.length || !series[0]?.data?.length;
    if (empty) {
        return (
            <div className="rounded-xl ">
                <div className="flex h-[335px] items-center justify-center">
                    <span className="text-sm text-gray-500 dark:text-gray-400">Live price data not available</span>
                </div>
            </div>
        );
    }

    const options: ApexOptions = {
        legend: {show: false, position: "top", horizontalAlign: "left"},
        colors: ["#465FFF"],
        chart: {
            fontFamily: "Outfit, sans-serif",
            height: 335,
            id: "area-datetime",
            type: "area",
            toolbar: {show: false}
        },
        stroke: {curve: "straight", width: [1]},
        dataLabels: {enabled: false},
        markers: {size: 0},
        xaxis: {
            type: "datetime",
            tickAmount: 10,
            axisBorder: {show: false},
            axisTicks: {show: false},
            tooltip: {enabled: false}
        },
        tooltip: {x: {format: "dd MMM yyyy"}},
        fill: {type: "gradient", gradient: {opacityFrom: 0, opacityTo: 0}},
        grid: {xaxis: {lines: {show: false}}, yaxis: {lines: {show: true}}},
        yaxis: {title: {text: "", style: {fontSize: "0px"}}}
    };

    return (
        <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="max-w-full overflow-x-auto custom-scrollbar">
                <div className="min-w-[1000px] xl:min-w-full">
                    <ReactApexChart options={options} series={series} type="area" height={335}/>
                </div>
            </div>
        </div>
    );
}
