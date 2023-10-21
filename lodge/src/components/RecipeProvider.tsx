import { ChangeEvent, useState } from "react"
import { useTranslation } from "react-i18next";

interface IRecipeProviderProps{
    
}

export default function RecipeProvider(_props: IRecipeProviderProps){
    const { t } = useTranslation();

    const [recipeStrings, setRecipeStrings] = useState<string[]>();
    const [_minutes, setMinutes] = useState<number>();

    const getRecipe = () => {
        // actually make a client and do le nice things
        setRecipeStrings(["Bobly"])
    }

    const updateMinutes = (e: ChangeEvent<HTMLInputElement>) => {
        const val: number = parseInt(e.target.value);
        if (val){
            setMinutes(val);
        }
      };

    return (
        <>
            <input placeholder="How long do you have to make some Succotash?" type="number" onChange={updateMinutes}></input>
            <button onClick={getRecipe}> {t("button.getrecipe")} </button>
            {recipeStrings && (
                <div>
                    This is where the recipe will appear, once it works.
                </div>
            )}
        </>
    )
}