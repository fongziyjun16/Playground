import { useRouter } from "next/router";
import React, { useEffect, useRef, useState } from "react";
import { Button, Card, Form, Input, InputRef, notification, Space } from "antd";
import type { FormProps } from "antd";
import { FaRegUser } from "react-icons/fa";
import { constructApiUrl } from "@/utils";
import { useAppDispatch } from "@/redux/hooks";
import { setIdentificationState } from "@/redux/slices/Identification";

interface FieldType {
    username: string;
}

export default function PortMainPage() {
    const router = useRouter();

    const dispatch = useAppDispatch();

    const [notificationApi, notificationContextHolder] = notification.useNotification();

    const [form] = Form.useForm();
    const usernameInputRef = useRef<InputRef>(null);
    const [confirmLoading, setConfirmLoading] = useState<boolean>(false);

    useEffect(() => {
        usernameInputRef.current!.focus({ cursor: "end" });
    }, []);

    const onFinish: FormProps<FieldType>["onFinish"] = values => {
        let { username } = values;
        username = username.trim();
        if (username.length === 0) {
            form.resetFields();
            notificationApi.error({
                message: "Invalid Input",
            });
            return;
        }
        setConfirmLoading(true);
        fetch(constructApiUrl(`/newSpeaker/${username}`), {
            method: "POST",
        })
            .then(resp => {
                if (resp.ok) {
                    resp.text().then(token => {
                        const parts = token.split("|");
                        if (parts.length != 4) {
                            notificationApi.error({
                                message: "Insecure Status",
                                description: "Try again later",
                            });
                        } else {
                            dispatch(
                                setIdentificationState({
                                    token,
                                    id: parts[3],
                                    username,
                                }),
                            );
                        }
                        router.replace(`/${username}`);
                    });
                } else {
                    notificationApi.error({
                        message: "Invalid Input",
                    });
                }
            })
            .catch(error => {
                console.log(error);
                notificationApi.error({
                    message: "Fail to Request",
                    description: "Try again later",
                });
            })
            .finally(() => setConfirmLoading(false));
    };

    return (
        <>
            {notificationContextHolder}
            <div style={{ alignItems: "center", display: "flex", justifyContent: "center", height: "100vh" }}>
                <Card>
                    <Space direction="vertical" style={{ width: "100%" }}>
                        <Form autoComplete="off" form={form} layout="inline" name="form" onFinish={onFinish}>
                            <Form.Item<FieldType>
                                name="username"
                                rules={[
                                    {
                                        message: "Invalid Input",
                                        pattern: /^(?!.*\s$)[a-zA-Z0-9_ ]+(?<!\s)$/,
                                        required: true,
                                    },
                                ]}
                            >
                                <Input allowClear ref={usernameInputRef} placeholder="Your Name" prefix={<FaRegUser />} />
                            </Form.Item>

                            <Form.Item>
                                <Button htmlType="submit" loading={confirmLoading} type="primary">
                                    Go to Chat
                                </Button>
                            </Form.Item>
                        </Form>
                        <span>Name Pattern: a-z, A-Z, 0-9, space, _, no space predix and suffix</span>
                    </Space>
                </Card>
            </div>
        </>
    );
}
