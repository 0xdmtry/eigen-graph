"use client";
import {useEffect, useRef, useState} from "react";

type Options = {
    url: string;
    symbol: string;
    maxPoints: number;
    heartbeatMs: number;
    backoff: { baseMs: number; maxMs: number };
    paused?: boolean;
};

export function useWebSocket(options?: Partial<Options>) {
    const opts: Options = {
        url: options?.url ?? "ws://localhost:8001/v1/stream/ws?symbol=ETH-USD",
        symbol: options?.symbol ?? "ETH-USD",
        maxPoints: options?.maxPoints ?? 3000,
        heartbeatMs: options?.heartbeatMs ?? 20000,
        backoff: {baseMs: options?.backoff?.baseMs ?? 1000, maxMs: options?.backoff?.maxMs ?? 30000},
        paused: options?.paused ?? false
    };

    const [data, setData] = useState<[number, number][]>([]);
    const wsRef = useRef<WebSocket | null>(null);
    const hbRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const roRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const delayRef = useRef<number>(opts.backoff.baseMs);
    const lastTsRef = useRef<number>(0);
    const mountedRef = useRef<boolean>(false);
    const pausedRef = useRef<boolean>(!!opts.paused);

    const clearTimers = () => {
        if (hbRef.current) clearTimeout(hbRef.current);
        if (roRef.current) clearTimeout(roRef.current);
        hbRef.current = null;
        roRef.current = null;
    };

    const scheduleReconnect = () => {
        if (!mountedRef.current || pausedRef.current) return;
        const jitter = Math.floor(Math.random() * opts.backoff.baseMs);
        const delay = Math.min(delayRef.current + jitter, opts.backoff.maxMs);
        roRef.current = setTimeout(connect, delay);
        delayRef.current = Math.min(delayRef.current * 2, opts.backoff.maxMs);
    };

    const resetHeartbeat = () => {
        if (hbRef.current) clearTimeout(hbRef.current);
        hbRef.current = setTimeout(() => {
            try {
                wsRef.current?.close();
            } catch {
            }
        }, opts.heartbeatMs);
    };

    const connect = () => {
        if (wsRef.current) wsRef.current.close();
        if (pausedRef.current) return;
        try {
            wsRef.current = new WebSocket(opts.url);
        } catch {
            scheduleReconnect();
            return;
        }

        wsRef.current.onopen = () => {
            delayRef.current = opts.backoff.baseMs;
            resetHeartbeat();
        };

        wsRef.current.onmessage = (event: MessageEvent) => {
            resetHeartbeat();
            try {
                const msg = JSON.parse(event.data as string) as { price: string; time: string };
                const t = new Date(msg.time).getTime();
                const p = Number.parseFloat(msg.price);
                if (!Number.isFinite(t) || !Number.isFinite(p)) return;
                if (t <= lastTsRef.current) return;
                lastTsRef.current = t;
                if (!pausedRef.current) {
                    setData((prev: [number, number][]) => {
                        const next: [number, number][] = prev.length >= opts.maxPoints ? [...prev.slice(1), [t, p]] : [...prev, [t, p]];
                        return next;
                    });
                }
            } catch (err) {
                console.error(err);
            }
        };

        wsRef.current.onerror = () => {
            scheduleReconnect();
        };

        wsRef.current.onclose = () => {
            clearTimers();
            if (mountedRef.current && !pausedRef.current) scheduleReconnect();
        };
    };

    useEffect(() => {
        mountedRef.current = true;
        pausedRef.current = !!opts.paused || (typeof document !== "undefined" && document.hidden);

        const onVis = () => {
            pausedRef.current = !!opts.paused || document.hidden;
            if (pausedRef.current) {
                try {
                    wsRef.current?.close();
                } catch {
                }
                clearTimers();
            } else {
                connect();
            }
        };

        document.addEventListener("visibilitychange", onVis, {passive: true});
        if (!pausedRef.current) connect();

        return () => {
            mountedRef.current = false;
            document.removeEventListener("visibilitychange", onVis);
            clearTimers();
            try {
                wsRef.current?.close();
            } catch {
            }
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [opts.url, opts.symbol, opts.maxPoints, opts.heartbeatMs, opts.backoff.baseMs, opts.backoff.maxMs, opts.paused]);

    return data;
}
