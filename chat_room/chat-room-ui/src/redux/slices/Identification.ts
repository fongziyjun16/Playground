import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/redux/store";

export interface Identification {
    token: string;
    id: string;
    username: string;
}

export const defaultIdentification = (): Identification => ({ token: "", id: "", username: "" });

export interface IdentificationState {
    value: Identification;
}

const initialState: IdentificationState = {
    value: defaultIdentification(),
};

export const IdentificationSlice = createSlice({
    name: "identification",
    initialState,
    reducers: {
        setIdentificationState: (state, action: PayloadAction<Identification>) => {
            state.value = action.payload;
        },

        clearIdentificationState: (state, _) => {
            state.value = defaultIdentification();
        },
    },
});

export const { setIdentificationState, clearIdentificationState } = IdentificationSlice.actions;

export const selectIdentification = (state: RootState) => state.identification.value;

export default IdentificationSlice.reducer;
