import { useState } from "react"
import { useTranslation } from "react-i18next";

interface ICounterProps{
    val: number
}

export default function Counter(props: ICounterProps){
    const { t } = useTranslation();

    const [value, setValue] = useState<number>(props.val)

    const increment = ()=>{
        setValue(value+1)
    }

    return (
        <>
            <button onClick={increment}> {t("counter.label")} ({t("counter",{count:value})}) {value} </button>
        </>
    )
}