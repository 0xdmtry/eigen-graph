import {canonicalSymbol, marketSymbolFor} from "@/features/tokens/market";
import {baseTokenCards} from "@/data/tokens";

describe("canonicalSymbol", () => {
    test("returns canonical case for known symbols", () => {
        for (const t of baseTokenCards) {
            expect(canonicalSymbol(t.symbol)).toBe(t.symbol);
            expect(canonicalSymbol(t.symbol.toLowerCase())).toBe(t.symbol);
            expect(canonicalSymbol(t.symbol.toUpperCase())).toBe(t.symbol);
        }
    });

    test("returns null for unknown symbols", () => {
        expect(canonicalSymbol("XYZ_UNKNOWN")).toBeNull();
    });
});

describe("marketSymbolFor", () => {
    test("preserves case and appends -USD for known symbols", () => {
        const sample = baseTokenCards.find(x => x.symbol.includes("ETH")) ?? baseTokenCards[0];
        const {stream} = marketSymbolFor(sample.symbol);
        expect(stream).toBe(`${sample.symbol}-USD`);
    });

    test("returns null stream when uiSymbol is null", () => {
        const {stream} = marketSymbolFor(null);
        expect(stream).toBeNull();
    });
});
