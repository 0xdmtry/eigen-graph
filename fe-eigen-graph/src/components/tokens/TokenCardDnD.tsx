"use client";
import React from "react";

type Props = {
    id: string;
    symbol: string;
    name: string;
    icon: string;
    tvl: string;
    operators: number;
    isActive: boolean;
    onSelect: () => void;
};

export default function TokenCardDnD(_: Props) {
    return <div/>;
}
