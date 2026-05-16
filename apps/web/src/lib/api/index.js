import ky from 'ky';
import z from 'zod';

export const api = ky.create({ baseUrl: 'http://localhost:5173/api/', retry: 0 });

/**
 * @template {z.ZodType} T
 * @param {T} type
 */
export function unwrapData(type) {
	return (
		z
			.object({ data: type })
			.strict()
			// @ts-expect-error
			.transform((v) => v.data)
	);
}
