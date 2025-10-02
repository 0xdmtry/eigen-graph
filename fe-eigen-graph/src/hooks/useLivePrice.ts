"use client";
import {useEffect, useRef, useState} from "react";
import type {LivePriceStatus} from "@/features/tokens/market";

type Options = {
    streamSymbol: string | null;
    urlBase?: string;
    path?: string;
    maxPoints?: number;
    heartbeatMs?: number;
    backoffBaseMs?: number;
    backoffMaxMs?: number;
    paused?: boolean;
};

type Point = [number, number];

export function useLivePrice(opts: Options) {
    const base = opts.urlBase ?? process.env.NEXT_PUBLIC_WS_BASE ?? "ws://localhost:8001";
    const path = opts.path ?? "/v1/stream/ws";
    const maxPoints = opts.maxPoints ?? 3000;
    const heartbeatMs = opts.heartbeatMs ?? 20000;
    const backoffBaseMs = opts.backoffBaseMs ?? 1000;
    const backoffMaxMs = opts.backoffMaxMs ?? 30000;

    const [points, setPoints] = useState<Point[]>([]);
    const [status, setStatus] = useState<LivePriceStatus>(opts.streamSymbol ? "idle" : "unavailable");
    const [error, setError] = useState<string | undefined>(undefined);

    const wsRef = useRef<WebSocket | null>(null);
    const hbRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const roRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const delayRef = useRef<number>(backoffBaseMs);
    const lastTsRef = useRef<number>(0);
    const mountedRef = useRef<boolean>(false);
    const pausedRef = useRef<boolean>(!!opts.paused);

    const bufRef = useRef<Point[]>([]);
    const rafRef = useRef<number | null>(null);

    const clearTimers = () => {
        if (hbRef.current) clearTimeout(hbRef.current);
        if (roRef.current) clearTimeout(roRef.current);
        hbRef.current = null;
        roRef.current = null;
    };

    const scheduleReconnect = () => {
        if (!mountedRef.current || pausedRef.current || !opts.streamSymbol) return;
        const jitter = Math.floor(Math.random() * backoffBaseMs);
        const delay = Math.min(delayRef.current + jitter, backoffMaxMs);
        roRef.current = setTimeout(connect, delay);
        delayRef.current = Math.min(delayRef.current * 2, backoffMaxMs);
    };

    const resetHeartbeat = () => {
        if (hbRef.current) clearTimeout(hbRef.current);
        hbRef.current = setTimeout(() => {
            try {
                wsRef.current?.close();
            } catch {
            }
        }, heartbeatMs);
    };

    const flush = () => {
        rafRef.current = null;
        const batch = bufRef.current;
        if (!batch.length) return;
        bufRef.current = [];
        setPoints(prev => {
            let next = prev.length ? (prev as Point[]).concat(batch) : batch;
            if (next.length > maxPoints) next = next.slice(-maxPoints);
            return next;
        });
    };

    const scheduleFlush = () => {
        if (rafRef.current != null) return;
        if (typeof window === "undefined") return;
        rafRef.current = window.requestAnimationFrame(flush);
    };

    const connect = () => {
        if (!opts.streamSymbol) return;
        const url = `${base}${path}?symbol=${encodeURIComponent(opts.streamSymbol)}`;
        if (wsRef.current) wsRef.current.close();
        if (pausedRef.current) return;

        try {
            wsRef.current = new WebSocket(url);
        } catch {
            setStatus("error");
            setError("ws_open_failed");
            scheduleReconnect();
            return;
        }

        setStatus("connecting");
        setError(undefined);

        wsRef.current.onopen = () => {
            delayRef.current = backoffBaseMs;
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
                    bufRef.current.push([t, p]);
                    if (status !== "live") setStatus("live");
                    scheduleFlush();
                }
            } catch (err) {
                console.error(err);
            }
        };

        wsRef.current.onerror = () => {
            setStatus("error");
            setError("ws_error");
            scheduleReconnect();
        };

        wsRef.current.onclose = () => {
            clearTimers();
            if (mountedRef.current && !pausedRef.current) scheduleReconnect();
        };
    };

    useEffect(() => {
        mountedRef.current = true;
        setPoints([]);
        bufRef.current = [];
        lastTsRef.current = 0;

        if (!opts.streamSymbol) {
            setStatus("unavailable");
            setError(undefined);
            return () => {
                mountedRef.current = false;
                clearTimers();
                try {
                    wsRef.current?.close();
                } catch {
                }
                if (rafRef.current != null && typeof window !== "undefined") window.cancelAnimationFrame(rafRef.current);
                rafRef.current = null;
            };
        }

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

        if (typeof document !== "undefined") document.addEventListener("visibilitychange", onVis, {passive: true});
        if (!pausedRef.current) connect();

        return () => {
            mountedRef.current = false;
            if (typeof document !== "undefined") document.removeEventListener("visibilitychange", onVis);
            clearTimers();
            try {
                wsRef.current?.close();
            } catch {
            }
            if (rafRef.current != null && typeof window !== "undefined") window.cancelAnimationFrame(rafRef.current);
            rafRef.current = null;
            bufRef.current = [];
        };
    }, [opts.streamSymbol, base, path, maxPoints, heartbeatMs, backoffBaseMs, backoffMaxMs, opts.paused]);

    return {points, status, error};
}
