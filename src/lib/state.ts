import { Config } from "./config.js";
import LoginScreen from "../screens/login.js";
import { Libs } from "./index.js";

export class State {
  private currentScreen: "home" | "login" = "login";

  constructor(config: Config) {
    this.currentScreen = config.inner.username === undefined ? "login" : "home";
  }

  public getScreenJSX(libs: Libs): React.JSX.Element {
    switch (this.currentScreen) {
      case "home":
        return LoginScreen({ libs });
      case "login":
        return LoginScreen({ libs });
    }
  }
}