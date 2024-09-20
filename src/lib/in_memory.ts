export class InMemory {
  private inner: Record<string, string> = {};

  public set(key: string, value: string) {
    this.inner[key] = value;
  }

  public get(key: string) {
    return this.inner[key];
  }
}