export class BaseClient {

    baseUrl: string;

    constructor(){
        this.baseUrl = "http://localhost:8000/api/v1";
    }

}

export function newSessionClient(): BaseClient {
    return  new BaseClient();
}