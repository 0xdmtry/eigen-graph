import {fetchAggregates} from "@/server/operators";

export async function GET() {
    try {
        const data = await fetchAggregates();
        return Response.json(data);
    } catch {
        return new Response("Upstream error", {status: 502});
    }
}
