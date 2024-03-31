import React, { PropsWithChildren } from "react";
import { SessionType, useSession } from "../context/session";
import { Navigate } from "react-router-dom";

export default function Protected(
    props: PropsWithChildren<object>
): React.JSX.Element {
    const session = useSession();

    if (session.sessionType == SessionType.NO_SESSION) {
        // user is not authenticated
        return <Navigate to="/login" />;
    }
    return <>{props.children}</>;
}
