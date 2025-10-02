import {baseTokenCards} from "@/data/tokens";

export type LivePriceStatus = "idle" | "connecting" | "live" | "unavailable" | "error";

const upperToCanonical = new Map(baseTokenCards.map(t => [t.symbol.toUpperCase(), t.symbol]));

const overrides: Record<string, string | null> = {};

export function canonicalSymbol(routeSymbol: string | null | undefined): string | null {
    if (!routeSymbol) return null;
    const key = routeSymbol.toUpperCase();
    return upperToCanonical.get(key) ?? null;
}

export function marketSymbolFor(uiSymbol: string | null | undefined): { stream: string | null } {
    if (!uiSymbol) return {stream: null};
    const override = overrides[uiSymbol];
    if (override === null) return {stream: null};
    if (typeof override === "string") return {stream: override};
    return {stream: `${uiSymbol}-USD`};
}
