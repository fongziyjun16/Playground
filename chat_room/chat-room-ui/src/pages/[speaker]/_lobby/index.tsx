import { useAppSelector } from "@/redux/hooks";
import { selectIdentification } from "@/redux/slices/Identification";
import { Card, notification } from "antd";
import WebSocketConnector from "@/pages/[speaker]/_lobby/WebSocketConnector";

export default function Lobby() {
    const identification = useAppSelector(selectIdentification);

    const [notificationApi, notificationContextHolder] = notification.useNotification();

    return (
        <>
            {notificationContextHolder}
            <div style={{ alignItems: "center", display: "flex", justifyContent: "center", height: "100vh" }}>
                <Card
                    extra={<WebSocketConnector />}
                    style={{ height: "90vh", width: "90vw" }}
                    title={identification.username}
                ></Card>
            </div>
        </>
    );
}
