"use client";

import React from "react";
import {SWRConfig} from "swr";

const staleTimeMs = Number(process.env.NEXT_PUBLIC_STALE_TIME_MS ?? "60000");

export function Providers({children}: { children: React.ReactNode }) {
    return (
        <SWRConfig
            value={{
                revalidateOnFocus: false,
                revalidateOnReconnect: false,
                dedupingInterval: staleTimeMs,
                fetcher: (url: string) => fetch(url).then(r => r.json()),
            }}
        >
            {children}
        </SWRConfig>
    );
}
