import {Client, ILauncherOptions} from 'minecraft-launcher-core';
import {InMemory} from '@lib/in_memory.js';
import {Config} from '@lib/config.js';
import path from 'path';
import axios from 'axios';
import syncFs, {promises as fs} from 'fs';

export class Launch {
	inner: Client;
	private authlibInjector = path.join(process.cwd(), 'authlib.jar');

	constructor(
		private inMemory: InMemory,
		private config: Config,
	) {
		this.inner = new Client();
	}

	get opts(): ILauncherOptions {
		return {
			authorization: {
				name: this.inMemory.get('name')!,
				uuid: this.inMemory.get('id')!,
				client_token: this.inMemory.get('clientToken')!,
				access_token: this.inMemory.get('accessToken')!,
				user_properties: {},
			},
			root: './minecraft',
			version: {
				number: this.config.inner.version,
				type: this.config.inner.type,
			},
			memory: {
				max: this.config.inner.memory_max,
				min: this.config.inner.memory_min,
			},
			customArgs: [
				`-javaagent:${this.authlibInjector}=${this.config.inner.yggdrasil_host}`,
				'-Dauthlibinjector.noShowServerName',
			],
			quickPlay: {
				type: 'multiplayer',
				identifier: this.config.inner.server,
			},
		};
	}

	public async launch() {
		await this.downloadAuthlibInjector();
		await this.inner.launch(this.opts);
	}

	public async downloadAuthlibInjector() {
		if (syncFs.existsSync(this.authlibInjector)) {
			return;
		}

		const response = await axios.get(this.config.inner.authlib, {
			responseType: 'arraybuffer',
		});
		const fileData = Buffer.from(response.data, 'binary');
		await fs.writeFile(this.authlibInjector, fileData);
	}
}
