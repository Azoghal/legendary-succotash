import { createContext, useContext } from "react";

export enum SessionType{
    NO_SESSION,
    USER,
}

export type ISession = {
    sessionType: SessionType;
    name: string;
    user_sub: string;
};

export const emptySession: ISession = { sessionType: SessionType.NO_SESSION, name: "", user_sub: "" };

const defaultSessionState = emptySession;

export const SessionContext = createContext(defaultSessionState);

export function useSession(): ISession {
    return useContext(SessionContext);
}
