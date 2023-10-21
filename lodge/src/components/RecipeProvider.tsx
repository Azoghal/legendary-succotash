import { ChangeEvent, useState } from "react"
import { useTranslation } from "react-i18next";

// import {ServotashClient} from "../../gen/prototash_pb_service.js" 
import {ServotashClient} from "../../gen/prototash_grpc_pb"
import grpc from "@grpc/grpc-js"
import { RecipeRequest, RecipeReply } from "../../gen/prototash_pb";

interface IRecipeProviderProps{
    
}

export default function RecipeProvider(_props: IRecipeProviderProps){
    const { t } = useTranslation();

    const [recipeStrings, setRecipeStrings] = useState<string[]>();
    const [_minutes, setMinutes] = useState<number>();

    const getRecipe = () => {
        // actually make a client and do le nice things
        var client = new ServotashClient('localhost:50051', grpc.credentials.createInsecure())
        const req: RecipeRequest = new RecipeRequest()
        req.setMaxtime(20);
        const bob: grpc.requestCallback<RecipeReply> = function(err, resp) {
            if (err){
                console.log(err)
                return
            }
            console.log(resp)
            setRecipeStrings(resp?.getInstructionsList())
        };
        client.recipe(req, bob)
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
                    {recipeStrings.map((s) => <span>{s}</span>)}
                </div>
            )}
        </>
    )
}