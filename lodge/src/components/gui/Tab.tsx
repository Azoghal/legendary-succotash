import * as React from "react";

interface ITabProps {
    title: string;
}

export default function Tab(props: ITabProps): React.JSX.Element {
    return <div className="tab">{props.title}</div>;
}
