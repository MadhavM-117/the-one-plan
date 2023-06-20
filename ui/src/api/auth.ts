import wretch from "wretch";
import { LoginRequest, SignupRequest, UserData } from "src/types";

export interface AuthApiInterface {
  whoAmI: () => Promise<UserData>;
  signup: (request: SignupRequest) => Promise<UserData>;
  login: (request: LoginRequest) => Promise<UserData>;
  logout: () => Promise<void>;
}

export const AuthApiFactory = (baseUrl: string): AuthApiInterface => {
  return {
    whoAmI: async () => {
      return await wretch(`${baseUrl}/auth/whoami`)
        .options({ credentials: "include" })
        .get()
        .json<UserData>();
    },
    signup: async (request: SignupRequest) => {
      return await wretch(`${baseUrl}/auth/signup`)
        .post(request)
        .json<UserData>();
    },
    login: async (request: LoginRequest) => {
      const { username, password } = request;
      return await wretch(`${baseUrl}/auth/login`)
        .auth(`Basic ${btoa(`${username}:${password}`)}`)
        .get()
        .json<UserData>();
    },
    logout: async () => {
      await wretch(`${baseUrl}/auth/logout`).get().res();
    },
  };
};
