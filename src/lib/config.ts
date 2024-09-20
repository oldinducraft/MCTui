import { z } from "zod";
import fs from "fs";

const CONFIG_SCHEMA = z.object({
  yggdrasil_host: z.string(),
  version: z.string(),
  username: z.string().optional(),
  password: z.string().optional(),
})

export class Config {
  inner!: z.infer<typeof CONFIG_SCHEMA>;

  constructor() {
    const file = fs.readFileSync("./settings.json", "utf8");
    this.inner = CONFIG_SCHEMA.parse(JSON.parse(file));
  }

  public save() {
    fs.writeFileSync("./settings.json", JSON.stringify(this.inner, null, 2));
  } 
}