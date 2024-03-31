import React, { PropsWithChildren } from "react";
import { SessionType, useSession } from "../context/session";
import * as Router from "react-router-dom";

export default function Protected(
    props: PropsWithChildren<object>
): React.JSX.Element {
    const session = useSession();

    if (session.sessionType == SessionType.NO_SESSION) {
        // user is not authenticated
        // TODO fix this
        Router.redirect("/login");
        return <></>;
    }
    return <>{props.children}</>;
}
