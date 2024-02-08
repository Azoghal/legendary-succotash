import { useState } from "react"
import { useTranslation } from "react-i18next";
import { IRecipe, newSuccotashClient } from "../services/succotash";



export default function RecipeButton(){
    const { t } = useTranslation();

    const [data, setData] = useState<IRecipe[]>([]);

    const hitApi = ()=>{
        newSuccotashClient()
        .list()
        .then(resp=>{
            console.log(resp);
            setData(resp.recipes);
        })
        .catch((e)=>console.error("failed to list succotash recipes: ", e))
    }

    return (
        <>
            <button onClick={hitApi}> {t("counter.label")} </button>
            <div>{data.map((r)=>(<>{r.instructions}</>))}</div>
        </>
    )
}