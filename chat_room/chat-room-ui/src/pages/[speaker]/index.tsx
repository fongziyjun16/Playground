import { useRouter } from "next/router";
import { useAppSelector } from "@/redux/hooks";
import { selectIdentification } from "@/redux/slices/Identification";
import { useEffect, useState } from "react";
import { constructApiUrl } from "@/utils";
import Loading from "@/pages/[speaker]/Loading";
import Lobby from "@/pages/[speaker]/_lobby";

export default function SpeakerMainPage() {
    const router = useRouter();

    const identification = useAppSelector(selectIdentification);
    const [pageLoading, setPageLoading] = useState<boolean>(true);

    useEffect(() => {
        verifyIdentification();
    }, []);

    const verifyIdentification = () => {
        fetch(constructApiUrl("/isNew"), {
            method: "GET",
            headers: { Authorization: `Bearer ${identification.token}` },
        })
            .then(resp => {
                if (!resp.ok) {
                    router.replace("/port");
                } else {
                    setPageLoading(false);
                }
            })
            .catch(error => router.replace("/port"));
    };

    return <>{pageLoading ? <Loading /> : <Lobby />}</>;
}
