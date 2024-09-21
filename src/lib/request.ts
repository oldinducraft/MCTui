import axios, {Axios} from 'axios';
import {Config} from './config.js';

// export async function getUUID(username: string): Promise<UUIDResult> {
//   const response = await instance.get(`/account/users/profiles/minecraft/${username}`);

//   return {
//     success: (response.status === 200) as true,
//     result: response.data
//   };
// }

export class Request {
	private inner: Axios;

	constructor(config: Config) {
		this.inner = axios.create({
			baseURL: `https://${config.inner.yggdrasil_host}`,
			validateStatus: () => true,
			timeout: 2000,
			timeoutErrorMessage: 'Request timed out',
		});
	}

	public async authenticate(
		username: string,
		password: string,
	): Promise<AuthenticateSuccess> {
		const res = await this.inner.post<AuthenticateResponse>(
			'/auth/authenticate',
			{
				username,
				password,
			},
		);

		if ('errorMessage' in res.data) {
			throw new Error(res.data.errorMessage);
		}

		return res.data;
	}

	public async profile(username: string): Promise<ProfileSuccess> {
		const res = await this.inner.get<ProfileResponse>(
			`/account/users/profiles/minecraft/${username}`,
		);

		if ('errorMessage' in res.data) {
			throw new Error(res.data.errorMessage);
		}

		return res.data;
	}
}

export interface AuthenticateSuccess {
	accessToken: string;
	clientToken: string;
}

export interface ProfileSuccess {
	name: string;
	id: string;
}

export interface ErrorResponse {
	error: string;
	errorMessage: string;
}

export type AuthenticateResponse = AuthenticateSuccess | ErrorResponse;
export type ProfileResponse = ProfileSuccess | ErrorResponse;
