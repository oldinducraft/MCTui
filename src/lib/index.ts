import {Config} from './config.js';
import {InMemory} from './in_memory.js';
import {Request} from './request.js';

export interface IncompleteLibs {
	config: Config;
	request: Request;
	inMemory: InMemory;
}

export type Libs = IncompleteLibs & {
	setScreen: (screen: Screens) => void;
};

export type Screens = 'home' | 'login' | 'authenticate' | 'account';
