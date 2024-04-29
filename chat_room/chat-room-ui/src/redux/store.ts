import { configureStore } from "@reduxjs/toolkit";
import IdentificationReducer from "@/redux/slices/Identification";

export const makeStore = () => {
    return configureStore({
        reducer: {
            identification: IdentificationReducer,
        },
    });
};

export type AppStore = ReturnType<typeof makeStore>;
export type RootState = ReturnType<AppStore["getState"]>;
export type AppDispatch = AppStore["dispatch"];
