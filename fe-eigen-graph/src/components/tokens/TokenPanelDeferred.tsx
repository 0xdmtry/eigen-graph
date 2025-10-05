"use client";
import React, {useEffect, useState} from "react";
import TokenPanelSkeleton from "./TokenPanelSkeleton";
import TokenPanelDnDStable from "./TokenPanelDnDStable";
import type {TableItem} from "@/types/operators";

export default function TokenPanelDeferred({tokens}: { tokens: Record<string, TableItem[]> }) {
    const [ready, setReady] = useState(false);
    useEffect(() => {
        const timer = setTimeout(() => setReady(true), 150);
        return () => clearTimeout(timer);
    }, []);
    return ready ? <TokenPanelDnDStable tokens={tokens}/> : <TokenPanelSkeleton/>;
}
