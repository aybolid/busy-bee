import z from "zod";

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
