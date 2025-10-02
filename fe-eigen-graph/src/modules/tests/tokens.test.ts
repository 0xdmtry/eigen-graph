import {isValidToken, normalizeToken, allTokens} from "@/modules/tokens";

test("normalizeToken and isValidToken", () => {
    const sample = allTokens()[0]?.symbol || "EIGEN";
    expect(isValidToken(sample.toLowerCase())).toBe(true);
    expect(normalizeToken(sample.toLowerCase())).toBe(sample.toUpperCase());
    expect(isValidToken("unknown")).toBe(false);
    expect(normalizeToken("unknown")).toBe("");
});
