"use client";
import React from "react";
import TokenPrice from "@/components/tokens/TokenPrice";
import {useLivePrice} from "@/hooks/useLivePrice";
import {marketSymbolFor} from "@/features/tokens/market";
import {useToken} from "@/context/TokenContext";

export default function TokenPriceLive() {
    const {selectedTokenSymbol} = useToken();
    const {stream} = marketSymbolFor(selectedTokenSymbol || null);
    const {points, status} = useLivePrice({streamSymbol: stream});

    if (!stream) {
        return (
            <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
                <div className="flex h-[335px] items-center justify-center">
                    <span
                        className="text-sm text-gray-500 dark:text-gray-400">Live price unavailable for {selectedTokenSymbol ?? "-"}</span>
                </div>
            </div>
        );
    }

    if (status === "error") {
        return (
            <div className="rounded-xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03]">
                <div className="flex h-[335px] items-center justify-center">
                    <span className="text-sm text-gray-500 dark:text-gray-400">Live price error</span>
                </div>
            </div>
        );
    }

    return <TokenPrice series={[{name: stream, data: points}]}/>;
}
