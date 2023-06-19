import { BASE_URL } from "src/constants";

export type ApiInterface = unknown;

const ApiCombiner = (_baseUrl = ""): ApiInterface => {
  return {};
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
