import React, {useState} from 'react';
import {Box} from 'ink';
import {IncompleteLibs, Libs, Screens} from './lib/index.js';
import LoginScreen from './screens/login.js';
import HomeScreen from './screens/home.js';
import AuthenticateScreen from './screens/authenticate.js';
import AccountScreen from './screens/account.js';

// type Item<V> = {
// 	key?: string;
// 	label: string;
// 	value: V;
// };

// const MENU: Item<string>[] = [
// 	{
// 		label: 'Start',
// 		value: 'start'
// 	},
// 	{
// 		label: 'Account',
// 		value: 'account'
// 	},
// 	{
// 		label: 'Exit',
// 		value: 'exit'
// 	}
// ];

function getScreenJSX(screen: Screens, libs: Libs) {
	switch (screen) {
		case 'home':
			return <HomeScreen libs={libs} />;
		case 'login':
			return <LoginScreen libs={libs} />;
		case 'authenticate':
			return <AuthenticateScreen libs={libs} />;
		case 'account':
			return <AccountScreen libs={libs} />;
	}
}

export default function App(props: {libs: IncompleteLibs}) {
	const [screen, setScreen] = useState<Screens>(
		props.libs.config.inner.username ? 'home' : 'login',
	);
	const libs = {...props.libs, setScreen};

	return (
		<Box
			height="100%"
			width="100%"
			alignItems="center"
			justifyContent="center"
			flexDirection="column"
		>
			{getScreenJSX(screen, libs)}
		</Box>
	);
}
