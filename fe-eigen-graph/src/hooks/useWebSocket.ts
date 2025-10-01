"use client";
import {useState, useEffect} from "react";

interface WebSocketMessage {
    price: string;
    time: string;
}

const WEBSOCKET_URL = "ws://localhost:8001/v1/stream/ws?symbol=ETH-USD";

export function useWebSocket() {
    const [data, setData] = useState<[number, number][]>([]);

    useEffect(() => {
        const socket = new WebSocket(WEBSOCKET_URL);

        socket.onmessage = (event) => {
            try {
                const message: WebSocketMessage = JSON.parse(event.data);
                const time = new Date(message.time).getTime();
                const price = parseFloat(message.price);
                setData((prevData) => [...prevData, [time, price]]);
            } catch (error) {
                // TODO: handle the fail
            }
        };

        return () => {
            socket.close();
        };
    }, []);

    return data;
}