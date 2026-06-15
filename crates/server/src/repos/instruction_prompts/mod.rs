mod fns;
mod types;

pub use fns::{
    InstructionPromptUpdateData, create_instruction_prompt, delete_instruction_prompt_by_id,
    get_instruction_prompt, get_instruction_prompts, get_instruction_prompts_by_ids,
    update_instruction_prompt_by_id,
};
pub use types::{
    InstructionPrompt, InstructionPromptId, InstructionPromptIds, InstructionPromptText,
    InstructionPromptTitle,
};
