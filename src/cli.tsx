#!/usr/bin/env node
import "reflect-metadata";
import React from 'react';
import App from './app.js';
import { withFullScreen } from 'fullscreen-ink';
import { State } from "./lib/state.js";
import { Config } from "./lib/config.js";
import { Libs } from "./lib/index.js";
import { Request } from "./lib/request.js";
import { InMemory } from "./lib/in_memory.js";

const config = new Config();
const state = new State(config);
const request = new Request(config);
const inMemory = new InMemory();
const libs: Libs = { config, state, request, inMemory }

withFullScreen(<App libs={libs} />).start();
 