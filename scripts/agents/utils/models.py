import os
import json
import logging
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from agents.utils.local_cache import local_cacher
import litellm
from tenacity import (
    before_sleep_log,
    retry,
    retry_if_not_exception_type,
    stop_after_attempt,
    wait_exponential,
)
from agents.utils.cache_control import set_cache_control

logger = logging.getLogger("litellm_model")

def extract_usage_info(usage):
    try:
        usage_info = {
            "prompt_tokens": usage.prompt_tokens,
            "completion_tokens": usage.completion_tokens,
            "total_tokens": usage.total_tokens,
            "cache_creation_input_tokens": usage.cache_creation_input_tokens,
            "cache_read_input_tokens": usage.cache_read_input_tokens,
            "cached_tokens": usage.prompt_tokens_details.cached_tokens,
        }
    except AttributeError:
        ### OpenAI no cache control
        usage_info = {
            "prompt_tokens": usage.prompt_tokens,
            "completion_tokens": usage.completion_tokens,
            "total_tokens": usage.total_tokens,
        }
    return usage_info

@dataclass
class LitellmModelConfig:
    model_name: str
    model_kwargs: dict[str, Any] = field(default_factory=dict)
    litellm_model_registry: Path | None = None


class LitellmModel:
    def __init__(self, **kwargs):
        self.config = LitellmModelConfig(**kwargs)
        self.cost = 0.0
        self.n_calls = 0
        self.max_context_length = 200000 - 64000 # 200K tokens - 64K tokens # TODO: get the max context length from the model
        if self.config.litellm_model_registry is not None:
            litellm.utils.register_model(json.loads(self.config.litellm_model_registry.read_text()))

    @retry(
        stop=stop_after_attempt(10),
        wait=wait_exponential(multiplier=1, min=4, max=60),
        before_sleep=before_sleep_log(logger, logging.WARNING),
        retry=retry_if_not_exception_type(
            (
                litellm.exceptions.UnsupportedParamsError,
                litellm.exceptions.NotFoundError,
                litellm.exceptions.PermissionDeniedError,
                litellm.exceptions.ContextWindowExceededError,
                litellm.exceptions.APIError,
                litellm.exceptions.AuthenticationError,
                KeyboardInterrupt,
            )
        ),
    )
    def _query(self, messages: list[dict[str, str]], **kwargs):
        try:
            return litellm.completion(
                model=self.config.model_name, messages=messages, **(self.config.model_kwargs | kwargs)
            )
        except litellm.exceptions.AuthenticationError as e:
            e.message += " You can permanently set your API key with `mini-extra config set KEY VALUE`."
            raise e

    @local_cacher("LOCAL_CACHE", cache_folder="./__local_cache__")
    def query_with_cache(self, messages: list[dict[str, str]], **kwargs):
        return self.query(messages, **kwargs)
    
    def query(self, messages: list[dict[str, str]], **kwargs) -> dict:
        response = self._query(messages, **kwargs)
        cost = litellm.cost_calculator.completion_cost(response)
        self.n_calls += 1
        self.cost += cost
        return {
            "content": response.choices[0].message.content or "",  # type: ignore
            "usage": extract_usage_info(response.usage),
            "cost": cost,
        }


class AnthropicModel(LitellmModel):
    """For the use of anthropic models, we need to add explicit cache control marks
    to the messages or we lose out on the benefits of the cache.
    Because break points are limited per key, we also need to rotate between different keys
    if running with multiple agents in parallel threads.
    """

    @local_cacher("LOCAL_CACHE", cache_folder="./__local_cache__")
    def query_with_cache(self, messages: list[dict], **kwargs):
        return self.query(messages, **kwargs)

    def query(self, messages: list[dict], **kwargs) -> dict:
        assert self.config.model_name in ["claude-sonnet-4-20250514", "claude-sonnet-4-5-20250929"], "[ERROR] Current only support claude-sonnet-4-20250514 and claude-sonnet-4-5-20250929"
        assert os.path.exists("./__secret__/claude.key"), "[ERROR] Anthropic API key is not set"
        with open("./__secret__/claude.key", "r") as f:
            api_key = f.read().strip()
        max_tokens = 64000
        assert api_key, "[ERROR] Anthropic API key is not set"
        return super().query(set_cache_control(messages), api_key=api_key, max_tokens=max_tokens, max_completion_tokens=max_tokens, **kwargs)


class OpenAIModel(LitellmModel):
    """For the use of anthropic models, we need to add explicit cache control marks
    to the messages or we lose out on the benefits of the cache.
    Because break points are limited per key, we also need to rotate between different keys
    if running with multiple agents in parallel threads.
    """

    @local_cacher("LOCAL_CACHE", cache_folder="./__local_cache__")
    def query_with_cache(self, messages: list[dict], **kwargs):
        return self.query(messages, **kwargs)

    def query(self, messages: list[dict], **kwargs) -> dict:
        assert self.config.model_name in ["gpt-5-2025-08-07", "gpt-4o-2024-11-20", "gpt-4.1-2025-04-14", "gpt-5-mini-2025-08-07"], "[ERROR] Current only support gpt-5-2025-08-07 and gpt-4o-2024-11-20"
        assert os.path.exists("./__secret__/openai.key"), "[ERROR] OpenAI API key is not set"
        with open("./__secret__/openai.key", "r") as f:
            api_key = f.read().strip()
        max_tokens = 400000
        assert api_key, "[ERROR] OpenAI API key is not set"
        if self.config.model_name == "gpt-4.1-2025-04-14":
            return super().query(messages, api_key=api_key, max_completion_tokens=32768, **kwargs)
        elif self.config.model_name == "gpt-4o-2024-11-20":
            return super().query(messages, api_key=api_key, max_completion_tokens=16384, **kwargs)
        else:
            return super().query(messages, api_key=api_key, max_tokens=max_tokens, max_completion_tokens=128000, **kwargs)
