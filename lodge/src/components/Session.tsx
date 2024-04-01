import React, { useCallback, useEffect, useState } from "react";
import * as Router from "react-router-dom";
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
import Login from "./Login";

export default function Session(): React.JSX.Element {
    // Consider if we can use session or local storage for the session
    const [session, setSession] = useState<ISession>();

    const loadData = useCallback(() => {
        newSpotifyExampleClient()
            .session_test()
            .then((user) => {
                if (user) {
                    setSession({
                        sessionType: SessionType.USER,
                        name: user.name,
                        user_sub: user.auth0subject,
                    });
                } else {
                    setSession(emptySession);
                    console.log("no user session");
                }
            })
            .catch((e) => console.error("failed to fetch user session", e));
    }, []);

    useEffect(() => {
        loadData();
    }, [loadData]);


    if (!session) {
        return <></>;
    }

    return (
        <SessionContext.Provider value={session}>
            <Router.Routes>
                <Router.Route path="/" Component={Landing} />
                <Router.Route path="/login" Component={Login} />
                <Router.Route path="/notlanding" Component={TestNotLanding} />
                <Router.Route
                    path="/secret/notlanding"
                    element={<Protected children={<TestNotLanding />} />}
                />
            </Router.Routes>
        </SessionContext.Provider>
    );
}
