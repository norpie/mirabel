export function formatTime(iso8601: string): string {
		const date = new Date(iso8601);
		const now = new Date();
		const sameDay = date.toDateString() === now.toDateString();

		const hours = String(date.getHours()).padStart(2, '0');
		const minutes = String(date.getMinutes()).padStart(2, '0');

		if (!sameDay) {
			const day = date.getDate();
			const month = date.toLocaleString('default', { month: 'short' });
			const year = date.getFullYear();

			if (year !== now.getFullYear()) {
				return `${month} ${day}, ${year}, ${hours}:${minutes}`;
			}

			return `${month} ${day}, ${hours}:${minutes}`;
		}

		return `${hours}:${minutes}`;
	}
