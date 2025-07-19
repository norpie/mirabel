use crate::prelude::*;

use std::sync::Arc;

use indoc::indoc;
use tera::{Context, Tera};

use crate::{agent::AgentResponse, driver::llm::Llm};

const PROMPT: &str = indoc! {"
    Generate a concise and descriptive title for the following session based on the provided messages and any existing title.

    The title should:
    - Capture the essence of the discussion.
    - Be suitable for quickly scrolling through a list of sessions.
    - If a specific Github/Gitlab issue is discussed, include the issue number in the title.
    - Be no longer than 8 words.
    - Be best-effort, you might get only a single messages and no existing title as context.

    The title should not:
    - include any message quotes or references to specific messages.
    - include any personal information or sensitive data.
    - include emoji or special characters.
    - include markdown formatting.
    - include assumptions about content that is not explicitly mentioned in the messages.

    Reply with only the title, without any additional text or formatting.

    <existing_title>
    {% if existing_title %}
    Existing title: {{ existing_title }}
    {% else %}
    No existing title provided.
    {% endif %}
    </existing_title>

    <messages>
    {% for message in messages %}
        {{ message.0 }}: {{ message.1 }}
    {% endfor %}
    </messages>
"};

pub async fn generate_title(
    llm: Arc<dyn Llm>,
    existing_title: Option<String>,
    messages: Vec<(String, String)>,
) -> Result<AgentResponse<String>> {
    let mut context = Context::new();
    context.insert("existing_title", &existing_title);
    context.insert("messages", &messages);
    let rendered = Tera::one_off(PROMPT, &context, false)?;
    let response = llm.generate(None, &rendered).await?;
    Ok(AgentResponse {
        response: response.generation,
        metadata: response.metadata,
    })
}
