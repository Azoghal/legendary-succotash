import React, { useEffect, useState } from "react";
import { t } from "i18next";
import SuccotashLogo from "../assets/logo.png";
import ExampleCard from "./gui/ExampleCard";
import LoginButton from "./gui/LoginButton";
import { SessionType, useSession } from "./context/session";
import { newSessionClient } from "../services/session";

export default function Landing(): React.JSX.Element {
    const session = useSession();

    const [authUrl, setAuthUrl] = useState<string>();

    useEffect(() => {
        newSessionClient()
            .getAuthUrl()
            .then((url) => {
                console.log("the url:", url);
                setAuthUrl(url);
            })
            .catch((e) => {
                console.error("error: ", e);
                setAuthUrl("");
            });
    }, []);

    return (
        <>
            <header className="c-header">
                <div className="c-header-left">
                    <a
                        href="https://en.wikipedia.org/wiki/Succotash"
                        target="_blank"
                    >
                        <img
                            src={SuccotashLogo}
                            className="logo"
                            alt="Succotash Logo"
                        />
                    </a>
                    <h1>{t("title.succotash")}</h1>
                </div>
                <div className="c-header-right">
                    <LoginButton />
                    <button
                        className="c-btn"
                        onClick={() => {
                            newSessionClient()
                                .getUser()
                                .then((user) => console.log("the user:", user))
                                .catch((e) => console.error("error: ", e));
                            console.log("the session", session);
                        }}
                    >
                        check session
                    </button>

                    {/* TODO: turn this into a hard link to another page, where that page requires user to be authenticated.
                        Will need session user to indicate this state. I think buttons that need spotify log in to work should be
                        green/greyed out in some way to indicate to the user what they can do without logging in. */}
                    {session.sessionType == SessionType.USER && (
                        <a href={authUrl} className="c-btn">
                            sign into spotify
                        </a>
                    )}

                    <a className="c-btn" href="/notlanding">
                        not landing
                    </a>
                    <a className="c-btn" href="/secret/notlanding">
                        secret
                    </a>
                </div>
            </header>
            <main className="c-page">
                {/* https://bulma.io/documentation/columns/options/ */}
                {/* The above has a cool example where we can have wider or narrower things in each row */}
                <div className="columns is-multiline is-centered card-holder">
                    <div className="column is-3" key={1}>
                        <ExampleCard
                            title={t("tool.card.title", { count: 1 })}
                        />
                    </div>
                </div>
            </main>
        </>
    );
}
