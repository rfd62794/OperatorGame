# OpenAgent Pattern Analysis Report

**Target:** `C:\Github\OpenAgent`
**Purpose:** Extracting battle-tested orchestration models to retrofit into `auto_analyze_screenshots.py`.

## 1. Model Routing & Initialization
OpenAgent does not use the `anthropic` or `openai` SDK native packages. Instead, it securely rolls a pure Python `requests` harness wrapped in `OpenRouterLLMAdapter` found inside `src\orchestration\openrouter_llm.py`.

```python
# C:\Github\OpenAgent\src\orchestration\openrouter_llm.py (Lines 383-388)
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://github.com/OpenAgent/OpenAgent",
            "X-Title": "OpenAgent v0.2",
        }
        url = f"{self.base_url.rstrip('/')}/chat/completions"
        response = requests.post(url, headers=headers, json=payload, timeout=self.timeout)
```

## 2. DeepSeek V3 & Cost Tracking
DeepSeek V3 is mapped explicitly into the orchestration pricing tiers with hyper-low costs.

```python
# C:\Github\OpenAgent\src\orchestration\openrouter_llm.py (Lines 74-81)
        pricing = {
            "deepseek/deepseek-chat": {"input": 0.00000014, "output": 0.00000014},
            "anthropic/claude-3.5-sonnet": {"input": 0.000003, "output": 0.000015},
        }
        rates = pricing.get(self.model, pricing["deepseek/deepseek-chat"])
```

## 3. Error Handling & Fallbacks
OpenAgent implements a robust `call_with_retries` architecture featuring exponential backoff, terminating down into a predefined mapping of fallback models.

```python
# C:\Github\OpenAgent\src\orchestration\openrouter_llm.py (Lines 209-212)
    FALLBACK_MODELS = {
        "deepseek/deepseek-chat": "google/gemini-2.0-flash-001",
        "google/gemini-2.0-flash-001": "deepseek/deepseek-chat",
    }

# (Lines 329-333)
        fallback = self.FALLBACK_MODELS.get(model)
        if fallback and fallback != model:
            self.logger.info(f"Attempting fallback model: {fallback}")
            return self.call_with_retries(model=fallback, ...)
```

## 4. Configuration Management
It defaults explicitly to `.env` using environment parsing inside the constructor.

```python
# C:\Github\OpenAgent\src\orchestration\openrouter_llm.py (Lines 223-225)
        self.api_key = api_key or os.getenv("OPENROUTER_API_KEY")
        if not self.api_key:
            raise LLMError("OPENROUTER_API_KEY not found in environment variables")
```

## Application to OperatorGame
Rather than depending on the `anthropic` SDK (which natively rejects OpenRouter OpenAI-formatted vision loops), we will drop the `anthropic` dependency entirely and use a pure `requests.post` harness mirroring OpenAgent! We will wire `deepseek/deepseek-chat` as the primary vision parser.
