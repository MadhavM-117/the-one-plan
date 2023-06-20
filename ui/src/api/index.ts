import { AuthApiFactory, AuthApiInterface } from "src/api/auth";
import { BASE_URL } from "src/constants";

export interface ApiInterface {
  auth: AuthApiInterface;
}

const ApiCombiner = (_baseUrl = ""): ApiInterface => {
  return {
    auth: AuthApiFactory(_baseUrl),
  };
};

export class ApiFactory {
  private static api?: ApiInterface;

  static getApi = (): ApiInterface => {
    if (ApiFactory.api === undefined) {
      ApiFactory.api = ApiCombiner(BASE_URL);
    }

    return ApiFactory.api as ApiInterface;
  };
}
