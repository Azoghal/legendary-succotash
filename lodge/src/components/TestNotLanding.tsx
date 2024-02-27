import React from "react";
import { t } from "i18next";
import SuccotashLogo from "../assets/logo.png";
import TabContainer from "./gui/TabContainer";

export default function TestNotLanding(): React.JSX.Element {
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
            </header>
            <main className="c-page">
                <TabContainer tabTitles={["first_tab", "second_tab"]} />
            </main>
        </>
    );
}
