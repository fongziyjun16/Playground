import { useRouter } from "next/router";
import { useAppSelector } from "@/redux/hooks";
import { selectIdentification } from "@/redux/slices/Identification";
import { useEffect } from "react";
import { Skeleton } from "antd";

export default function RootEntry() {
    const router = useRouter();

    const identification = useAppSelector(selectIdentification);

    useEffect(() => {
        if (identification.token.length === 0 || identification.username.length === 0) {
            router.replace("/port");
        } else {
            router.replace(`/${identification.username}`);
        }
    }, []);

    return (
        <>
            <Skeleton active />
        </>
    );
}
