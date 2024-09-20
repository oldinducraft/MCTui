import { Config } from "./config.js";
import { InMemory } from "./in_memory.js";
import { Request } from "./request.js";
import { State } from "./state.js";

export interface Libs {
  config: Config,
  state: State,
  request: Request,
  inMemory: InMemory
}