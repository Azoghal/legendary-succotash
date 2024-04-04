import React from "react";
import { t } from "i18next";
import SuccotashLogo from "../assets/logo.png";
import ExampleCard from "./gui/ExampleCard";
import LoginButton from "./gui/LoginButton";
import { useSession } from "./context/session";
import { newSessionClient } from "../services/session";

export default function Landing(): React.JSX.Element {
    const session = useSession();

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
