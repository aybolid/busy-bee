import z from 'zod';

/**
 * @template {z.ZodType} T
 * @param {T} type `zod` schema of the value to unwrap.
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
