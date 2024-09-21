import {Client, ILauncherOptions} from 'minecraft-launcher-core';
import {InMemory} from './in_memory.js';
import {Config} from './config.js';

export class Launch {
	inner: Client;

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
				`-javaagent:/home/thets/Dev/launcher/authlib.jar=${this.config.inner.yggdrasil_host}`,
				'-Dauthlibinjector.noShowServerName',
			],
			quickPlay: {
				type: 'multiplayer',
				identifier: this.config.inner.server,
			},
		};
	}

	public async launch() {
		await this.inner.launch(this.opts);
	}
}
