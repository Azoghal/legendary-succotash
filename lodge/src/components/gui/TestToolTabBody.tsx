import { useMemo, useState } from "react";
import TabContainer from "./TabContainer";

const tabTitles = ["Tool", "Settings"];

export default function TestToolTabBody(): JSX.Element {
    const [activeTab, setActiveTab] = useState<string>(tabTitles[0]);

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
                    </div>
                );
            case tabTitles[1]:
                return <div>This one's not got a card in it</div>;
        }
    }, [activeTab]);

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
