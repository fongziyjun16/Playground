import { AppProps } from "next/app";
import { NextPage } from "next";
import { ReactElement, ReactNode, useRef } from "react";
import { Provider } from "react-redux";
import { AppStore, makeStore } from "@/redux/store";
import "./globals.css";

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
    getLayout?: (page: ReactElement) => ReactNode;
};

type AppPropsWithLayout = AppProps & {
    Component: NextPageWithLayout;
};

export default function App({ Component, pageProps }: AppPropsWithLayout) {
    const storeRef = useRef<AppStore>();
    if (!storeRef.current) {
        storeRef.current = makeStore();
    }
    const getLayout = Component.getLayout ?? (page => page);
    return <Provider store={storeRef.current}>{getLayout(<Component {...pageProps} />)}</Provider>;
}
