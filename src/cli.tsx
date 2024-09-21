#!/usr/bin/env node
import React from 'react';
import App from './app.js';
import {withFullScreen} from 'fullscreen-ink';
import {Config} from './lib/config.js';
import {IncompleteLibs} from './lib/index.js';
import {Request} from './lib/request.js';
import {InMemory} from './lib/in_memory.js';

const config = new Config();
const request = new Request(config);
const inMemory = new InMemory();
const libs: IncompleteLibs = {config, request, inMemory};

withFullScreen(<App libs={libs} />).start();
