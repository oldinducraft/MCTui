import React from 'react';
import { Box } from 'ink';
import { Libs } from './lib/index.js';

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

export default function App(props: { libs: Libs }) {
	// const handleSelect = (item: Item<string>) => {
	// 	switch (item.value) {
	// 		case "exit":
	// 			process.exit(0);
	// 	}
	// };

	return <Box height="100%" width="100%" alignItems="center" justifyContent="center" flexDirection='column'>
		{ props.libs.state.getScreenJSX(props.libs) }
	</Box>;
}
