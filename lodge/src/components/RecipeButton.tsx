import { useState } from "react";
import { useTranslation } from "react-i18next";
import { newSuccotashClient } from "../services/succotash";
import { Recipes } from "../gen/types/Recipes";

export default function RecipeButton() {
    const { t } = useTranslation();

    const [data, setData] = useState<Recipes>();

    const hitApi = () => {
        newSuccotashClient()
            .list()
            .then((resp) => {
                console.log(resp);
                setData(resp);
            })
            .catch((e) =>
                console.error("failed to list succotash recipes: ", e)
            );
    };

    return (
        <>
            <button onClick={hitApi} className="c-btn">
                {" "}
                {t("counter.label")}{" "}
            </button>
            <div>{data && data.recipes.map((r) => <>{r.instructions}</>)}</div>
        </>
    );
}
