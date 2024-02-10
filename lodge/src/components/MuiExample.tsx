import { Button } from "@mui/base";
import { useState } from "react";

export default function(): JSX.Element{

    const [text, setText] = useState("I'm a test button")

    const handleTestClick = ()=>{
        setText("Ouch! that hurt :(")
    }  

    return (<Button onClick={handleTestClick}>{text}</Button>)
}