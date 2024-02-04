import { useState } from "react"
import { useTranslation } from "react-i18next";

export default function RecipeButton(){
    const { t } = useTranslation();

    const [value, setValue] = useState<string>("")

    const hitApi = ()=>{
        fetch("http://127.0.0.1:8000/api/v1/recipes").then((resp)=>{
            resp.json()
        }).then((resp)=>{
            console.log(resp);
            setValue(JSON.stringify(resp))
        })
    }

    return (
        <>
            <button onClick={hitApi}> {t("counter.label")} </button>
            {value && <>{value}</>}
        </>
    )
}