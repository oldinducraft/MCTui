import {Box, Text} from 'ink';
import React, {useEffect} from 'react';
import KeyHint, {KeyHintDef} from '@components/key_hint.js';
import {Libs} from '@lib/index.js';
import BigText from 'ink-big-text';
import Spinner from 'ink-spinner';

const KEY_DEFS: KeyHintDef[] = [{key: '<Ctrl+C>', action: 'Exit'}];

export default function AuthenticateScreen(props: {libs: Libs}) {
	useEffect(() => {
		(async () => {
			if (!props.libs.config.hasCredentials()) {
				props.libs.setScreen('login');
				return;
			}

			const username = props.libs.config.inner.username!;
			const password = props.libs.config.inner.password!;

			const res = await props.libs.request
				.authenticate(username, password)
				.catch(() => {
					props.libs.setScreen('login');
				});

			if (!res) {
				props.libs.setScreen('login');
				return;
			}

			const profile = await props.libs.request.profile(username).catch(() => {
				props.libs.setScreen('login');
			});

			if (!profile) {
				props.libs.setScreen('login');
				return;
			}

			props.libs.inMemory.set('accessToken', res.accessToken);
			props.libs.inMemory.set('clientToken', res.clientToken);
			props.libs.inMemory.set('name', profile.name);
			props.libs.inMemory.set('id', profile.id);

			props.libs.setScreen('home');
		})();
	}, [props.libs.config.inner.username, props.libs.config.inner.password]);

	return (
		<Box
			flexDirection="column"
			borderStyle="bold"
			width="100%"
			height="100%"
			alignItems="center"
			justifyContent="space-between"
		>
			<BigText text="oldindcraft" />
			<Box flexDirection="column" gap={1}>
				<Spinner type="material" />
				<Text>Trying to log in...</Text>
			</Box>
			<KeyHint defs={KEY_DEFS} />
		</Box>
	);
}
