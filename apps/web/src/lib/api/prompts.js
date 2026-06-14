import z from "zod";
import { unwrapData } from "./common";
import { formatDate } from "$lib/formats";

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
    .transform((data) => ({
        ...data,
        formattedCreatedAt: () => formatDate(data.created_at),
        formattedUpdatedAt: () => formatDate(data.updated_at),
        formattedVersion: () => `Version ${data.version}`,
    }))
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

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 *
 * @returns {Promise<Array<SystemPrompt>>}
 */
export async function getSystemPrompts(ky) {
    const json = await ky.get(`system_prompts`).json();

    return unwrapData(z.array(systemPromptSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: SystemPromptId } }} payload Request payload.
 *
 * @returns {Promise<SystemPrompt>}
 */
export async function getSystemPrompt(ky, payload) {
    const json = await ky.get(`system_prompts/${payload.params.id}`).json();

    return unwrapData(systemPromptSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: SystemPromptId } }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function deleteSystemPrompt(ky, payload) {
    await ky.delete(`system_prompts/${payload.params.id}`);
}

export const updateSystemPromptJsonSchema = z
    .object({
        version: systemPromptVersionSchema,
        title: z.string().trim().min(1).max(255).optional(),
        text: z.string().trim().min(1).optional(),
    })
    .strict();

/** @typedef {z.infer<typeof updateSystemPromptJsonSchema>} UpdateSystemPromptJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: SystemPromptId }, json: UpdateSystemPromptJson }} payload Request payload.
 *
 * @returns {Promise<SystemPrompt>}
 */
export async function updateSystemPrompt(ky, payload) {
    const json = await ky
        .patch(`system_prompts/${payload.params.id}`, {
            json: updateSystemPromptJsonSchema.parse(payload.json),
        })
        .json();

    return unwrapData(systemPromptSchema).parse(json);
}

//////////////////////////////////////////

export const instructionPromptIdSchema = z.uuidv7().brand("instructionPromptId");

/** @typedef {z.infer<typeof instructionPromptIdSchema>} InstructionPromptId */

export const instructionPromptVersionSchema = z.int().positive().brand("instructionPromptVersion");

/** @typedef {z.infer<typeof instructionPromptVersionSchema>} InstructionPromptVersion */

export const instructionPromptSchema = z
    .object({
        id: instructionPromptIdSchema,
        created_at: z.coerce.date(),
        updated_at: z.coerce.date(),
        title: z.string(),
        text: z.string(),
        version: instructionPromptVersionSchema,
    })
    .strict()
    .transform((data) => ({
        ...data,
        formattedCreatedAt: () => formatDate(data.created_at),
        formattedUpdatedAt: () => formatDate(data.updated_at),
        formattedVersion: () => `Version ${data.version}`,
    }))
    .readonly();

/** @typedef {z.infer<typeof instructionPromptSchema>} InstructionPrompt */

export const createInstructionPromptJsonSchema = z
    .object({
        title: z.string().trim().min(1).max(255),
        text: z.string().trim().min(1).max(500),
    })
    .strict();

/** @typedef {z.infer<typeof createInstructionPromptJsonSchema>} CreateInstructionPromptJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ json: CreateInstructionPromptJson }} payload Request payload.
 *
 * @returns {Promise<InstructionPrompt>}
 */
export async function createInstructionPrompt(ky, payload) {
    const json = await ky
        .post(`instruction_prompts`, {
            json: createInstructionPromptJsonSchema.parse(payload.json),
        })
        .json();

    return unwrapData(instructionPromptSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 *
 * @returns {Promise<Array<InstructionPrompt>>}
 */
export async function getInstructionPrompts(ky) {
    const json = await ky.get(`instruction_prompts`).json();

    return unwrapData(z.array(instructionPromptSchema)).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: InstructionPromptId } }} payload Request payload.
 *
 * @returns {Promise<InstructionPrompt>}
 */
export async function getInstructionPrompt(ky, payload) {
    const json = await ky.get(`instruction_prompts/${payload.params.id}`).json();

    return unwrapData(instructionPromptSchema).parse(json);
}

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: InstructionPromptId } }} payload Request payload.
 *
 * @returns {Promise<void>}
 */
export async function deleteInstructionPrompt(ky, payload) {
    await ky.delete(`instruction_prompts/${payload.params.id}`);
}

export const updateInstructionPromptJsonSchema = z
    .object({
        version: instructionPromptVersionSchema,
        title: z.string().trim().min(1).max(255).optional(),
        text: z.string().trim().min(1).max(500).optional(),
    })
    .strict();

/** @typedef {z.infer<typeof updateInstructionPromptJsonSchema>} UpdateInstructionPromptJson */

/**
 * @param {import('ky').KyInstance} ky `KyInstance` to use.
 * @param {{ params: { id: InstructionPromptId }, json: UpdateInstructionPromptJson }} payload Request payload.
 *
 * @returns {Promise<InstructionPrompt>}
 */
export async function updateInstructionPrompt(ky, payload) {
    const json = await ky
        .patch(`instruction_prompts/${payload.params.id}`, {
            json: updateSystemPromptJsonSchema.parse(payload.json),
        })
        .json();

    return unwrapData(instructionPromptSchema).parse(json);
}
