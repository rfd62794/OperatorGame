# OpenAgent Architectural Deep Dive
**Target Repository:** `C:\Github\OpenAgent`  
**Purpose:** Comprehensive dissection of Agent Handling, Model Orchestration, and System Methodologies.

---

## 1. Core Methodologies & System Architecture
OpenAgent abandons "zero-shot" monolithic coding in favor of a strictly decoupled, **Multi-Pass Granular Pipeline**. It treats software engineering as an assembly line rather than a single LLM prompt.

### The Four-Pass Granular Pipeline
As defined in `GRANULAR_PLANNING_ARCHITECTURE.md`, OpenAgent achieves high-fidelity code modifications through sequential state transformations:
1. **Pass 1 (Design):** The `DesignerAgent` parses the codebase and outputs high-level abstract goals (`Roadmap`).
2. **Pass 2 (Analysis):** The `PythonProjectAnalyzer` builds a deterministic AST/Regex map of the project (`CodebaseMap`), locating classes, methods, and precise line numbers for insertion points.
3. **Pass 3 (Elaboration):** The `TaskElaboratorAgent` merges the abstract Roadmap with the CodebaseMap. It outputs highly concrete `ElaboratedTasks` containing exact `file:line` modification snippets.
4. **Pass 4 (Verification):** The `VerificationAgent` acts as a discriminator, ensuring the elaborated steps won't break the build before any code is actually executed by the `ExecutorAgent`.

**Philosophy:** OpenAgent operates asynchronously from the developer. It heavily utilizes CLI-driven modular phases (`inventory`, `analyze`, `assess-docs`, `assess-tests`) meaning the LLM's context window is strictly protected. It only acts on localized JSON artifacts rather than attempting to read the entire repository at once.

---

## 2. Agent Handling
OpenAgent utilizes a strict Object-Oriented **Class Inheritance Model** for Agent identities, rooted in `src/agents/base_agent.py`.

### Identity & Isolation
Every agent (`AuditorAgent`, `CoderAgent`, `DesignerAgent`) must inherit from `BaseAgent`. 
- **Decoupled LLM Adapters:** Agents do not manage their own API connections. They are injected with an `OpenRouterLLMAdapter` upon instantiation. If an adapter isn't provided, they spin up a default connection.
- **Strict Role Boundaries:** Each agent is initialized with a hardcoded `name`, `description`, and `system_prompt`. This prevents "role bleed" where an auditor accidentally tries to write code.
- **Tool Registration:** Agents have a `self.tools` array mapped via `register_tools()`, ensuring tools are scoped strictly to the agent that needs them.

### Cross-Agent Communication
Agents do not talk to each other directly in natural language. They communicate entirely through structured JSON interfaces (e.g., `CodebaseMap`, `ElaboratedTask`). This enforces deterministic handoffs and allows the overarching `Orchestrator` to catch and correct validation errors between agents.

---

## 3. Model Handling & Orchestration
Model handling in OpenAgent is ruthlessly optimized for resilience and cost-efficiency. It entirely bypasses heavy proprietary SDKs (like `anthropic` or `openai`) and utilizes a pure Python `requests` harness found in `src\orchestration\openrouter_llm.py`.

### Hyper-Aggressive Cost Economics
Instead of routing everything to expensive frontier models, OpenAgent defaults heavily to `deepseek/deepseek-chat` for baseline structural analysis. It maintains an internal hardcoded USD pricing dictionary and intercepts the `usage` payload from OpenRouter to continuously track `cost_usd`. This data is actively spooled into `logs/cost_tracking.jsonl` allowing the CLI (`openagent costs`) to monitor budget burn rates dynamically.

### Autonomous Error Recovery
The `OpenRouterLLMAdapter.call_with_retries()` method is the backbone of the entire model handler:
1. **Exponential Backoff:** If OpenRouter throws a 429 (Rate Limit) or 500/502 (Overload), the adapter sleeps for `1s * (2^attempt)` before retrying natively.
2. **Fallback Cascades:** If the maximum retries fail on the primary model, it explicitly drops down to a `FALLBACK_MODELS` mapping:
   - DeepSeek V3 fails ➡️ Fallback to `google/gemini-2.0-flash-001`.
   - Gemini fails ➡️ Fallback to DeepSeek.
3. **Model Decoupling:** By abstracting the HTTP request into a unified `OpenRouter` wrapper, the agents are completely blind to *which* model is actually executing the prompt. The Orchestrator can swap DeepSeek for Claude mid-pipeline without breaking the Agent's code loop.
