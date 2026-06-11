import z from "zod";
import { unwrapData } from "./common";

export const systemPromptIdSchema = z.uuidv7().brand("systemPromptId");

/** @typedef {z.infer<typeof systemPromptIdSchema>} SystemPromptId */

export const systemPromptVersionSchema = z.int().positive().brand("systemPromptVersion");

/** @typedef {z.infer<typeof systemPromptVersionSchema>} SystemPromptVersion */

export const systemPromptSchema = z
    .object({
        id: systemPromptIdSchema,
        created_at: z.coerce.date(),
        updated_at: z.coerce.date(),
        title: z.string(),
        text: z.string(),
        version: systemPromptVersionSchema,
    })
    .strict()
    .readonly();

/** @typedef {z.infer<typeof systemPromptSchema>} SystemPrompt */

export const createSystemPromptJsonSchema = z
    .object({
        title: z.string().trim().min(1).max(255),
        text: z.string().trim().min(1),
    })
    .strict();

/** @typedef {z.infer<typeof createSystemPromptJsonSchema>} CreateSystemPromptJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ json: CreateSystemPromptJson }} payload Request payload.
 *
 * @returns {Promise<SystemPrompt>}
 */
export async function createSystemPrompt(ky, payload) {
    const json = await ky
        .post(`system_prompts`, {
            json: createSystemPromptJsonSchema.parse(payload.json),
        })
        .json();

    return unwrapData(systemPromptSchema).parse(json);
}
