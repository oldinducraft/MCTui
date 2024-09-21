import {Box, Text, useInput} from 'ink';
import React from 'react';
import KeyHint, {KeyHintDef} from '../components/key_hint.js';
import {Libs} from '../lib/index.js';
import BigText from 'ink-big-text';

const KEY_DEFS: KeyHintDef[] = [
	{key: '<Tab>', action: 'Return to home'},
	{key: '<Delete>', action: 'Log out (and exit)'},
	{key: '<Ctrl+C>', action: 'Exit'},
];

export default function AccountScreen(props: {libs: Libs}) {
	useInput(async (_input, key) => {
		if (key.tab) {
			props.libs.setScreen('home');
		}

		if (key.delete) {
			props.libs.config.inner.username = undefined;
			props.libs.config.inner.password = undefined;
			props.libs.config.save();
			process.exit(0);
		}
	});

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
			<Text>Logged in as</Text>
			<BigText font="slick" text={props.libs.inMemory.get('name')!} />
			<KeyHint defs={KEY_DEFS} />
		</Box>
	);
}
