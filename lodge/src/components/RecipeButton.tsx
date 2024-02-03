import { useState } from "react"
import { useTranslation } from "react-i18next";

export default function RecipeButton(){
    const { t } = useTranslation();

    const [value, setValue] = useState<string>("")

    const hitApi = ()=>{
        setValue("bobly")
    }

    return (
        <>
            <button onClick={hitApi}> {t("counter.label")} </button>
            {value && <>{value}</>}
        </>
    )
}