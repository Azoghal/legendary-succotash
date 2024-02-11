import { Recipes } from "../gen/types/Recipes";

class SuccotashClient {
  async list(): Promise<Recipes> {
    // TDOD get base url from env or context
    return fetch("http://127.0.0.1:8000/api/v1/recipes")
      .then((response) => response.json())
      .then((json) => json as Recipes);
  }
}

export function newSuccotashClient(): SuccotashClient {
  return new SuccotashClient();
}
