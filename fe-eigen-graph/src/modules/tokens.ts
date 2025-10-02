import {baseTokenCards} from "@/data/tokens";
import {TokenCardType} from "@/types/tokens";

const tokenSet = new Set(baseTokenCards.map(t => t.symbol.toUpperCase()));

export function normalizeToken(symbol: string | null | undefined): string {
    if (!symbol) return "";
    const u = symbol.toUpperCase();
    return tokenSet.has(u) ? u : "";
}

export function isValidToken(symbol: string | null | undefined): boolean {
    return normalizeToken(symbol) !== "";
}

export function allTokens(): TokenCardType[] {
    return baseTokenCards;
}
