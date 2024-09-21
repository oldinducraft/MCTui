import React from 'react';
import {Box, Text} from 'ink';

type ProgressBarProps = {
	text: string;
	current: number;
	total: number;
	width?: number;
};

const ProgressBar: React.FC<ProgressBarProps> = ({
	text,
	current,
	total,
	width = 20,
}) => {
	// Calculate progress as a value between 0 and 1, and limit it to a maximum of 1
	const progress = total > 0 ? Math.min(current / total, 1) : 0;
	const filledWidth = Math.round(progress * width);
	const emptyWidth = width - filledWidth;

	const filledBar = '█'.repeat(filledWidth);
	const emptyBar = '░'.repeat(emptyWidth);

	// Calculate percentage of progress
	const percentage = Math.round((current / total) * 100);

	return (
		<Box>
			<Text>
				{text} {filledBar + emptyBar} {percentage}% ({current}/{total})
			</Text>
		</Box>
	);
};

export default ProgressBar;
