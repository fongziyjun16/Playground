export const constructApiUrl = (endpoint: string): string => {
    return `${process.env.API_URL}${endpoint}`;
};
