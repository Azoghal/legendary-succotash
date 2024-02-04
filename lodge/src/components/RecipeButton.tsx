import { useState } from "react"
import { useTranslation } from "react-i18next";
import { newSuccotashClient } from "../services/succotash";


export default function RecipeButton(){
    const { t } = useTranslation();

    const [value, _setValue] = useState<string>("")
    const [data, setData] = useState<string[]>([]);

    const hitApi = ()=>{
        newSuccotashClient().list().then(resp=>setData(resp.recipes)).catch((e)=>console.error("failed to list succotash recipes: ", e))
    }

    return (
        <>
            <button onClick={hitApi}> {t("counter.label")} </button>
            {value && <>{value}</>}
            <div>{data.map((r)=>(<>{r}</>))}</div>
        </>
    )
}