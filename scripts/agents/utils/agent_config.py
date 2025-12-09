from dataclasses import dataclass
@dataclass
class AgentConfig:
    # The default settings are the bare minimum to run the agent. Take a look at the config files for improved settings.
    system_template: str = "You are a helpful assistant that can do anything."
    instance_template: str = (
        "Your task: {{task}}. Please reply with a single shell command in triple backticks. "
        "To finish, the first line of the output of the shell command must be 'MINI_SWE_AGENT_FINAL_OUTPUT'."
    )
    timeout_template: str = (
        "The last command <command>{{action['action']}}</command> timed out and has been killed.\n"
        "The output of the command was:\n <output>\n{{output}}\n</output>\n"
        "Please try another command and make sure to avoid those requiring interactive input."
    )
    format_error_template: str = "Please always provide EXACTLY ONE action in triple backticks."
    action_observation_template: str = "Observation: {{output}}"
    step_limit: int = 0
    cost_limit: float = 5.0

class NonTerminatingException(Exception):
    """Raised for conditions that can be handled by the agent."""

class FormatError(NonTerminatingException):
    """Raised when the LM's output is not in the expected format."""

class ExecutionTimeoutError(NonTerminatingException):
    """Raised when the action execution timed out."""

class TerminatingException(Exception):
    """Raised for conditions that terminate the agent."""

class Submitted(TerminatingException):
    """Raised when the LM declares that the agent has finished its task."""

class LimitsExceeded(TerminatingException):
    """Raised when the agent has reached its cost or step limit."""