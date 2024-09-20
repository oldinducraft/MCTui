import axios, { Axios } from "axios";
import { Config } from "./config.js";

export class Request {
  private inner: Axios;

  constructor(config: Config) {
    this.inner = axios.create({
      baseURL: `https://${config.inner.yggdrasil_host}`,
      validateStatus: () => true,
      timeout: 2000,
      timeoutErrorMessage: "Request timed out",
    });
  }

  public async authenticate(username: string, password: string): Promise<AuthenticateSuccess> {
    const res = await this.inner.post<AuthenticateResponse>("/auth/authenticate", {
      username, password
    });

    if ("error" in res.data) {
      throw new Error(res.data.errorMessage);
    }

    return res.data
  }
}

export interface AuthenticateSuccess {
  accessToken: string;
  clientToken: string;
}

export type AuthenticateResponse = AuthenticateSuccess | {
  error: string;
  errorMessage: string;
}