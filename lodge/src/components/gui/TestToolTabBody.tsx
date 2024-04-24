import { useCallback, useMemo, useState } from "react";
import TabContainer from "./TabContainer";
import { newSpotifyExampleClient } from "../../services/spotifyExample";

const tabTitles = ["Tool", "Settings"];

export default function TestToolTabBody(): JSX.Element {
    const [activeTab, setActiveTab] = useState<string>(tabTitles[0]);

    const [currentPlaying, setCurrentPlaying] = useState<string>();
    const [bobbis, setBobbis] = useState<string>();

    const getCurrentPlaying = useCallback(() => {
        newSpotifyExampleClient()
            .getCurrentlyPlaying()
            .then((s) => setCurrentPlaying(s.title))
            .catch((e) => console.error("failed to get current playing", e));
    }, []);

    const getAccessDetail = useCallback(() => {
        newSpotifyExampleClient()
            .getTempTokenDetail()
            .then((s) => setBobbis(s.title))
            .catch((e) => console.error("failed to get bobbis", e));
    }, []);

    const body = useMemo(() => {
        switch (activeTab) {
            default:
            case tabTitles[0]:
                return (
                    <div className="card">
                        <div className="card__title">
                            a card that lives in the tab body
                        </div>
                        <div className="card__body">the text of the card</div>
                        <div>
                            <button
                                className="c-btn"
                                onClick={getCurrentPlaying}
                            >
                                Get current playing
                            </button>
                            {currentPlaying ?? ""}
                        </div>
                        <div>
                            <button className="c-btn" onClick={getAccessDetail}>
                                Get Access Detail
                            </button>
                            {bobbis ?? ""}
                        </div>
                    </div>
                );
            case tabTitles[1]:
                return <div>This one's not got a card in it</div>;
        }
    }, [activeTab, bobbis, currentPlaying, getAccessDetail, getCurrentPlaying]);

    return (
        <TabContainer
            tabTitles={tabTitles}
            activeTab={activeTab}
            onTabchange={setActiveTab}
        >
            {body}
        </TabContainer>
    );
}
