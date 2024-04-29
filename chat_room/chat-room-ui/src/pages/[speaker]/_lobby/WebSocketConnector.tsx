import { useAppSelector } from "@/redux/hooks";
import { selectIdentification } from "@/redux/slices/Identification";
import { useEffect, useState } from "react";
import { Button, Space } from "antd";
import { GoDotFill } from "react-icons/go";
import { initializeWSClient, wsEventEmitter } from "@/utils/websocket";

export default function WebSocketConnector() {
    const identification = useAppSelector(selectIdentification);

    const [connected, setConnected] = useState<boolean>(false);

    wsEventEmitter.on("connected", () => setConnected(true));
    wsEventEmitter.on("disconnected", () => setConnected(false));

    useEffect(() => {
        initializeWSClient();
    }, []);

    return (
        <>
            <Button type="text">
                {connected ? (
                    <Space>
                        <div style={{ display: "flex", alignItems: "center" }}>
                            <GoDotFill color={"green"} />
                        </div>
                        <span>Connected</span>
                    </Space>
                ) : (
                    <Space>
                        <div style={{ display: "flex", alignItems: "center" }}>
                            <GoDotFill color={"red"} />
                        </div>
                        <span>Disconnected</span>
                    </Space>
                )}
            </Button>
        </>
    );
}
