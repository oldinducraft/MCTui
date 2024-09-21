import {Box, Text, useInput} from 'ink';
import React from 'react';
import KeyHint, {KeyHintDef} from '@components/key_hint.js';
import TextInput from 'ink-text-input';
import Spinner from 'ink-spinner';
import {Libs} from '@lib/index.js';

const KEY_DEFS: KeyHintDef[] = [
	{key: '<Enter>', action: 'Submit'},
	{key: '<Tab>', action: 'Focus on the next input'},
	{key: '<Ctrl+C>', action: 'Exit'},
];

export default function LoginScreen(props: {libs: Libs}) {
	const [loading, setLoading] = React.useState(false);
	const [errored, setErrored] = React.useState<undefined | string>(undefined);

	const [username, setUsername] = React.useState('');
	const [password, setPassword] = React.useState('');
	const [active, setActive] = React.useState<'username' | 'password'>(
		'username',
	);

	useInput(async (_input, key) => {
		if (key.tab) {
			setActive(active === 'username' ? 'password' : 'username');
		}

		if (key.return) {
			setErrored(undefined);
			setLoading(true);

			const res = await props.libs.request
				.authenticate(username, password)
				.catch((err: Error) => {
					setErrored(err.message);
				});
			setLoading(false);

			if (res) {
				props.libs.inMemory.set('accessToken', res.accessToken);
				props.libs.inMemory.set('clientToken', res.clientToken);

				props.libs.config.inner.username = username;
				props.libs.config.inner.password = password;
				props.libs.config.save();

				props.libs.setScreen('home');
			}
		}
	});

	return (
		<Box
			flexDirection="column"
			borderStyle="bold"
			width="60%"
			height="60%"
			alignItems="center"
			justifyContent="space-between"
		>
			<Text backgroundColor="red" color="white">
				{' '}
				🔒You need to login first🔒{' '}
			</Text>
			{loading ? (
				<Text>
					<Spinner type="dots" /> Trying to log in...
				</Text>
			) : (
				<></>
			)}
			{errored ? (
				<Text color="redBright">Failed to log in: {errored}</Text>
			) : (
				<></>
			)}
			<Box
				flexGrow={1}
				borderStyle="bold"
				width="100%"
				alignItems="center"
				justifyContent="flex-start"
				padding={2}
				flexDirection="column"
			>
				<Box>
					<Text color={active === 'username' ? 'yellowBright' : 'white'}>
						Username:{' '}
					</Text>
					<TextInput
						value={username}
						onChange={setUsername}
						showCursor={true}
						focus={active === 'username'}
					/>
				</Box>

				<Box>
					<Text color={active === 'password' ? 'yellowBright' : 'white'}>
						Password:{' '}
					</Text>
					<TextInput
						value={password}
						onChange={setPassword}
						showCursor={true}
						focus={active === 'password'}
						mask="*"
					/>
				</Box>
			</Box>
			<KeyHint defs={KEY_DEFS} />
		</Box>
	);
}
