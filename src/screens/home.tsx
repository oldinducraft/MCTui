import {Box} from 'ink';
import React from 'react';
import KeyHint, {KeyHintDef} from '../components/key_hint.js';
import {Libs} from '../lib/index.js';
import SelectInput from 'ink-select-input';
import BigText from 'ink-big-text';

const KEY_DEFS: KeyHintDef[] = [
	{key: '<↑/↓>', action: 'Select entry'},
	{key: '<Enter>', action: 'Enter selected entry'},
	{key: '<Ctrl+C>', action: 'Exit'},
];

type Item<V> = {
	key?: string;
	label: string;
	value: V;
};

const MENU: Item<string>[] = [
	{
		label: 'Start',
		value: 'start',
	},
	{
		label: 'Account',
		value: 'account',
	},
	{
		label: 'Exit',
		value: 'exit',
	},
];

export default function HomeScreen(props: {libs: Libs}) {
	const handleSelect = (item: Item<string>) => {
		switch (item.value) {
			case 'exit':
				process.exit(0);
			case 'account':
				props.libs.setScreen('account');
				break;
			case 'start':
				props.libs.setScreen('start');
				break;
		}
	};

	if (!props.libs.inMemory.isLoggedIn()) {
		props.libs.setScreen('authenticate');
		return <></>;
	}

	return (
		<Box
			flexDirection="column"
			alignItems="center"
			width="100%"
			height="100%"
			justifyContent="space-between"
		>
			<BigText text="oldindcraft" />
			<SelectInput items={MENU} onSelect={handleSelect} />
			<KeyHint defs={KEY_DEFS} />
		</Box>
	);
}
