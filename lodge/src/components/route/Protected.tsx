import React, { PropsWithChildren, useEffect } from "react";
import { SessionType, useSession } from "../context/session";
import { useNavigate } from "react-router-dom";
// import { Redirect } from "react-router";

export default function Protected(
    props: PropsWithChildren<object>
): React.JSX.Element {
    const session = useSession();
    const navigate = useNavigate();

    useEffect(() => {
        console.log(session);
        if (session.sessionType == SessionType.NO_SESSION) {
            // user is not authenticated
            // TODO fix this.
            console.log("should redirect to login");
            navigate("/login");
        }
    }, [session, navigate]);

    return (
        <>{session.sessionType != SessionType.NO_SESSION && props.children}</>
    );
}
