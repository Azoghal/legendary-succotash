import * as React from "react";
import Tab from "./Tab";
import TabBody from "./TabBody";

interface ITabContainerProps {
    tabTitles: string[];
    // components: JSX.Element[];
}

export default function TabContainer(props: ITabContainerProps): JSX.Element {
    return (
        <div className="tab-container">
            <div className="tab-container__tabs">
                {/* show all the tabs */}
                {props.tabTitles.map((s) => (
                    <Tab title={s} />
                ))}
            </div>
            <div className="tab-container__body">
                <TabBody />
            </div>
        </div>
    );
}
