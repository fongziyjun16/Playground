import WebSocket from "isomorphic-ws";
import mitt from "mitt";

type WSEvents = {
    connected: void;
    disconnected: void;
};

export const wsEventEmitter = mitt<WSEvents>();

let wsClient: WebSocket | null = null;

export const initializeWSClient = () => {
    if (wsClient !== null) {
        return;
    }
    wsClient = new WebSocket(`${process.env.WS_URL}`);
    wsClient.onopen = () => wsEventEmitter.emit("connected");
    wsClient.onclose = () => wsEventEmitter.emit("disconnected");
    wsClient.onerror = console.log;
};
