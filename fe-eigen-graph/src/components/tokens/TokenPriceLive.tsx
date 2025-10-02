"use client";
import React from "react";
import TokenPrice from "@/components/tokens/TokenPrice";
import {useWebSocket} from "@/hooks/useWebSocket";

export default function TokenPriceLive() {
    const data = useWebSocket();
    const series = [{name: "ETH-USD", data}];
    return <TokenPrice series={series}/>;
}
