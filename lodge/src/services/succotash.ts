
// TODO definitely generate these
export interface Recipes {
    recipes: IRecipe[]
}

export interface IRecipe{
    id: number;
    title: string;
    instructions: string;
}

// TODO would be nice if we could generate these
class SuccotashClient {

    async list(): Promise<Recipes>{
        // TDOD get url from env?
        return fetch("http://127.0.0.1:8000/api/v1/recipes")
        .then(response => response.json())
        .then(json=>json as Recipes)
    }
}

export function newSuccotashClient(): SuccotashClient {
    return new SuccotashClient
}