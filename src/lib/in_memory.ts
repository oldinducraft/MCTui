export interface InMemoryData {
	accessToken?: string;
	clientToken?: string;
	name?: string;
	id?: string;
}

export class InMemory {
	private inner: InMemoryData = {};

	public set<T extends keyof InMemoryData>(key: T, value: InMemoryData[T]) {
		this.inner[key] = value;
	}

	public get<T extends keyof InMemoryData>(key: T): InMemoryData[T] {
		return this.inner[key];
	}

	public isLoggedIn(): boolean {
		return (
			!!this.get('accessToken') &&
			!!this.get('clientToken') &&
			!!this.get('name') &&
			!!this.get('id')
		);
	}
}
