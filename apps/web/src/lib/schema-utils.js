/**
 * @template {import('zod').ZodType} T
 * @param {T} schema
 * @returns {number | null}
 */
export function getMaxLengthConstraint(schema) {
    // If wrapped (optional, nullable, etc.), unwrap it recursively
    if ("unwrap" in schema && typeof schema.unwrap === "function") {
        return getMaxLengthConstraint(schema.unwrap());
    }

    if ("maxLength" in schema && typeof schema.maxLength === "number") {
        return schema.maxLength;
    }

    return null;
}
