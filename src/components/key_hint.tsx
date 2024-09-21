import React from 'react';
import {Text} from 'ink';

export interface KeyHintDef {
	key: string;
	action: string;
}

export interface Props {
	defs: KeyHintDef[];
}

export default function KeyHint(props: Props) {
	const hints = props.defs.map(def => (
		<>
			<Text color="yellow">{def.key}</Text>
			<Text> {def.action}</Text>
		</>
	));
	const delimiter = <Text color="gray"> | </Text>;

	const result = hints.flatMap((item, index) =>
		index === hints.length - 1 ? [item] : [item, delimiter],
	);

	return <Text>{result}</Text>;
}
