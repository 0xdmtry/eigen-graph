"use client";
import React from "react";
import TokenCardSkeleton from "./TokenCardSkeleton";

export default function TokenPanelSkeleton() {
    return (
        <div className="rounded-2xl border-gray-800 bg-white/[0.03] p-4 sm:p-6 h-[350px]">
            <div className="h-[75px]"></div>
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
                {Array.from({length: 6}).map((_, i) => (
                    <TokenCardSkeleton key={i}/>
                ))}
            </div>
        </div>
    );
}
