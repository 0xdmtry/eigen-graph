"use client";

import useSWR from "swr";
import type {ApiResponse} from "@/types/operators";

type Source = "fallback" | "network";
type Wrapped = { payload: ApiResponse; fetchedAt: number; source: Source };

const defaultStaleTimeMs = Number(process.env.NEXT_PUBLIC_STALE_TIME_MS ?? "60000");

export function useAggregates(opts?: { fallbackData?: ApiResponse; staleTimeMs?: number }) {
    const key = "/api/operators/aggregates";
    const staleTimeMs = opts?.staleTimeMs ?? defaultStaleTimeMs;

    const fetcher = async (url: string): Promise<Wrapped> => {
        const res = await fetch(url);
        const json = (await res.json()) as ApiResponse;
        return {payload: json, fetchedAt: Date.now(), source: "network"};
    };

    const fallbackWrapped: Wrapped | undefined = opts?.fallbackData
        ? {payload: opts.fallbackData, fetchedAt: Date.now(), source: "fallback"}
        : undefined;

    const swr = useSWR<Wrapped>(key, fetcher, {
        fallbackData: fallbackWrapped,
        dedupingInterval: staleTimeMs,
        revalidateOnFocus: false,
        revalidateOnReconnect: false,
    });

    return {
        data: swr.data?.payload as ApiResponse | undefined,
        meta: {fetchedAt: swr.data?.fetchedAt ?? 0, source: (swr.data?.source ?? "fallback") as Source},
        isValidating: swr.isValidating,
        mutate: swr.mutate,
    };
}
