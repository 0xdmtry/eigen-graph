import {ApiResponse} from "@/types/operators";

export const REVALIDATE_SECONDS = 60;

function validate(data: any) {
    if (!data || typeof data !== "object" || typeof data.byToken !== "object") throw new Error("Invalid API response");
    for (const k of Object.keys(data.byToken)) {
        const s = data.byToken[k];
        if (!s || !Array.isArray(s.table) || !Array.isArray(s.bar) || !Array.isArray(s.graph)) throw new Error("Invalid API response slice");
    }
}

export async function fetchAggregates(): Promise<ApiResponse> {
    const base = process.env.API_URL;
    if (!base) throw new Error("API_URL is not configured");
    const r = await fetch(`${base}/v1/operators/aggregates`, {next: {revalidate: REVALIDATE_SECONDS}});
    if (!r.ok) throw new Error("Failed to fetch operators");
    const data = (await r.json()) as ApiResponse;
    validate(data);
    return data;
}
