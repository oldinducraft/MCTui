import {Config} from '@lib/config.js';
import {InMemory} from '@lib/in_memory.js';
import {Launch} from '@lib/launch.js';
import {Request} from '@lib/request.js';

export interface IncompleteLibs {
	config: Config;
	request: Request;
	inMemory: InMemory;
	launch: Launch;
}

export type Libs = IncompleteLibs & {
	setScreen: (screen: Screens) => void;
};

export type Screens = 'home' | 'login' | 'authenticate' | 'account' | 'start';
