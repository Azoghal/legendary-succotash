import React from "react";
import { t } from "i18next";
import SuccotashLogo from "../assets/logo.png";
import ExampleCard from "./gui/ExampleCard";
import LoginButton from "./gui/LoginButton";

export default function Landing(): React.JSX.Element {
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
