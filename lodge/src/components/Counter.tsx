import { useState } from "react"

interface ICounterProps{
    val: number
}

export default function Counter(props: ICounterProps){
    const [count, setCount] = useState<number>(props.val)

    const increment = ()=>{
        setCount(count+1)
    }

    return (
        <>
            <button onClick={increment}> Press me (also turn me into an i18n string) {count} </button>
        </>
    )
}