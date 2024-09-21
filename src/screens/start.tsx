import {Box, Text, useInput} from 'ink';
import React, {useEffect, useState} from 'react';
import KeyHint, {KeyHintDef} from '@components/key_hint.js';
import {Libs} from '@lib/index.js';
import ProgressBar from '@components/progress.js';
import BigText from 'ink-big-text';

const KEY_DEFS: KeyHintDef[] = [{key: '<Esc>', action: 'Exit'}];

export interface Task {
	type: string;
	task: number;
	total: number;
}

export default function StartScreen(props: {libs: Libs}) {
	const [output, setOutput] = React.useState<string[]>([]);
	const [tasks, setTasks] = useState<{[key: string]: Task}>({});

	useInput(async (_input, key) => {
		if (key.escape) {
			process.exit(0);
		}
	});

	useEffect(() => {
		props.libs.launch.inner.on('progress', (data: Task) => {
			setTasks(tasks => ({
				...tasks,
				[data.type]: data,
			}));
		});

		props.libs.launch.inner.on('debug', (message: string) => {
			setOutput(prev => [...prev, message.trim()]);
		});

		props.libs.launch.inner.on('data', (message: string) => {
			setOutput(prev => [...prev, message.trim()]);
		});

		props.libs.launch.launch();
	}, []);

	return (
		<Box
			flexDirection="column"
			alignItems="center"
			width="100%"
			height="100%"
			justifyContent="space-between"
		>
			<BigText text="starting..." />
			<Box
				flexDirection="column"
				gap={2}
				justifyContent="center"
				alignItems="center"
			>
				<Box flexDirection="column" alignItems="flex-start" flexGrow={1}>
					<Text wrap="truncate">{output.slice(-10).join('\n')}</Text>
				</Box>
				<Box flexDirection="column" alignItems="center" justifyContent="center">
					{Object.entries(tasks).map(([key, task]) => (
						<ProgressBar
							key={key}
							text={task.type}
							current={task.task}
							total={task.total}
							width={80}
						/>
					))}
				</Box>
			</Box>
			<KeyHint defs={KEY_DEFS} />
		</Box>
	);
}
