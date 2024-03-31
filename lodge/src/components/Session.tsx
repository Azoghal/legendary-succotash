import React, { useCallback, useEffect, useState } from "react";
import * as Router from "react-router";
import Landing from "./Landing";
import TestNotLanding from "./TestNotLanding";
import { newSpotifyExampleClient } from "../services/spotifyExample";
import {
    ISession,
    SessionContext,
    SessionType,
    emptySession,
} from "./context/session";
import Protected from "./route/Protected";

export default function Session(): React.JSX.Element {
    const [session, setSession] = useState<ISession>(emptySession);

    const loadData = useCallback(() => {
        newSpotifyExampleClient()
            .session_test()
            .then((user) =>
                setSession({
                    sessionType: SessionType.USER,
                    name: user.name,
                    user_sub: user.auth0subject,
                })
            )
            .catch((e) => console.error("no user session", e));
    }, []);

    useEffect(() => {
        loadData();
    }, [loadData]);

    return (
        <SessionContext.Provider value={session}>
            <Router.Routes>
                <Router.Route path="/" Component={Landing} />
                <Router.Route path="/notlanding" Component={TestNotLanding} />
                <Router.Route
                    path="/secret/notlanding"
                    element={<Protected children={<TestNotLanding />} />}
                />
            </Router.Routes>
        </SessionContext.Provider>
    );
}