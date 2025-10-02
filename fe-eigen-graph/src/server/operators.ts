import {ApiResponse} from "@/types/operators";

export const REVALIDATE_SECONDS = 60;

export async function fetchAggregates(): Promise<ApiResponse> {
    const base = process.env.API_URL;
    if (!base) throw new Error("API_URL is not configured");
    const r = await fetch(`${base}/v1/operators/aggregates`, {next: {revalidate: REVALIDATE_SECONDS}});
    if (!r.ok) throw new Error("Failed to fetch operators");
    return r.json();
}
