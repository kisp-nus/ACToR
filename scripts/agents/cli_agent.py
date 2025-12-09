import os
import platform
import re
import subprocess
from dataclasses import asdict
from jinja2 import Template
from agents.utils.local_env import LocalEnvironment
from agents.utils.models import LitellmModel, AnthropicModel
from agents.utils.agent_config import AgentConfig, NonTerminatingException, TerminatingException, FormatError, ExecutionTimeoutError, LimitsExceeded, Submitted
import yaml
import os
from pathlib import Path
import json
import time

CONFIG_FILE = "./scripts/agents/cli_agent.yml"
ASK_USER_CONFIRMATION = False
MAX_TOKENS_PER_QUERY = 10000
class MyCLIAgent:
    def __init__(self, agent_id: int, model: LitellmModel | AnthropicModel, env: LocalEnvironment, sandbox_dir: str, log_path: str, checkpoint_path: str):
        self.agent_id = agent_id
        self.config = AgentConfig(**yaml.safe_load(Path(CONFIG_FILE).read_text())['agent'])
        self.messages: list[dict] = []
        self.model = model
        self.env = env
        self.sandbox_dir = sandbox_dir
        self.log_path = log_path
        self.checkpoint_path = checkpoint_path
        self.stop_sign = False
        assert checkpoint_path is not None, "checkpoint_path is required"
        if os.path.exists(checkpoint_path):
            try:
                checkpoint = json.load(open(checkpoint_path))
                self.messages = checkpoint["messages"]
                assert len(self.messages) >= 2 and self.messages[0]["role"] == "system" and self.messages[1]["role"] == "user", "[ERROR] Invalid checkpoint file"
                self.total_cost = checkpoint["total_cost"]
                self.total_tokens = checkpoint["total_tokens"]

                return
            except Exception as e:
                print(e)
        
        ### check if dir exist
        if not os.path.exists(os.path.dirname(checkpoint_path)):
            os.makedirs(os.path.dirname(checkpoint_path))
        json.dump({"messages": [], "total_cost": 0}, open(checkpoint_path, "w"), indent=4)

        self.messages = []
        self.total_cost = 0
        self.total_tokens = 0

    def render_template(self, template: str, **kwargs) -> str:
        cs = asdict(self.config) | asdict(self.env.config) | asdict(self.model.config) | platform.uname()._asdict()
        return Template(template).render(**kwargs, **cs, **os.environ)

    def add_message(self, role: str, content: str):
        # Log the observation message
        with open(self.log_path, "a") as log_file:
            log_file.write("==============================================\n")
            log_file.write(f"[INFO] Role: {role}\n")
            log_file.write(f"[INFO] Content: {content}\n")
        
        self.messages.append({"role": role, "content": content})

        assert os.path.exists(self.checkpoint_path), f"[ERROR] Checkpoint file {self.checkpoint_path} does not exist"
        new_checkpoint = {
            "messages": self.messages,
            "total_cost": self.total_cost,
            "total_tokens": self.total_tokens,
        }
        json.dump(new_checkpoint, open(self.checkpoint_path, "w"), indent=4)

    def run(self, task: str) -> tuple[str, str]:
        """Run step() until agent is finished. Return exit status & message"""
        
        if self.messages == []:
            self.add_message("system", self.config.system_template)
            self.add_message("user", self.render_template(self.config.instance_template, task=task))
        if self.messages[-1]["role"] == "assistant": ### ensure the last message is from user
            try:
                observation = self.get_observation({"content": self.messages[-1]["content"]})
                self.add_message("user", observation)
            except TerminatingException as e:
                return type(e).__name__, str(e)

        while True:
            try:
                self.step()
                time.sleep(1)
            except NonTerminatingException as e:
                self.add_message("user", str(e))
            except TerminatingException as e:
                return type(e).__name__, str(e)

    def step(self) -> dict:
        """Query the LM, execute the action, return the observation."""
        query_response = self.query() ### first get the response from the model
        self.add_message("assistant", query_response["content"])
        observation = self.get_observation(query_response) ### then get the observation from the environment
        self.add_message("user", observation) ### give the observation back to the model

    def query(self) -> dict:
        """Query the model and return the response."""
        if 5.0 <= self.total_cost or 0 < self.model.max_context_length - MAX_TOKENS_PER_QUERY <= self.total_tokens:
            raise LimitsExceeded()
        response = self.model.query(self.messages)
        self.total_cost += response["cost"]
        self.total_tokens = response["usage"]["total_tokens"]
        with open(self.log_path, "a") as log_file:
            log_file.write("==============================================\n")
            log_file.write(f"[INFO] Used tokens: {response['usage']}\n")
            log_file.write(f"[INFO] Cost of the last query: {response['cost']}\n")
            log_file.write(f"[INFO] Total Cost Until Now: {self.total_cost}\n")
            
        return response

    def sanitiy_check(self, action: str) -> bool:
        """Check if the action is safe to execute."""
        if "unsafe {" in action:
            raise FormatError("[ERROR] Detected `unsafe` in the action. You are not allowed to use `unsafe` code.")
        if "::RefCell" in action:
            raise FormatError("[ERROR] Detected `::RefCell` in the action. You are not allowed to use `::RefCell` in your code.")
        if "ffi::" in action:
            raise FormatError("[ERROR] Detected `ffi::` in the action. You are not allowed to use `ffi::` in your code.")
        if "::Rc" in action:
            raise FormatError("[ERROR] Detected `::Rc` in the action. You are not allowed to use `::Rc` in your code.")
        if "::Arc" in action:
            raise FormatError("[ERROR] Detected `::Arc` in the action. You are not allowed to use `::Arc` in your code.")
        if "::Mutex" in action:
            raise FormatError("[ERROR] Detected `::Mutex` in the action. You are not allowed to use `::Mutex` in your code.")
        return True

    def get_observation(self, response: dict) -> dict:
        """Execute the action and return the observation."""
        action = self.parse_action(response)

        ### sanitiy check
        self.sanitiy_check(action["action"])

        ### Ask user for confirmation before executing command
        if ASK_USER_CONFIRMATION:
            print(f"\nProposed command: {action['action']}")
            confirmation = input("Execute this command? [y/N] ").lower()
            if confirmation == 'y': ### yes and continue
                output = self.execute_action(action)
            elif confirmation == 'ys': ### yes and stop
                output = self.execute_action(action)
                raise Submitted()
            else: ### stop
                output = {"output": "Command execution cancelled by user. You should end the task."}
                raise Submitted()
        else:
            output = self.execute_action(action)
            
        # Log the output for recovery and checking purposes
        with open(self.log_path, "a") as log_file:
            log_file.write("==============================================\n")
            log_file.write(f"Action: {action['action']}\n")
            log_file.write(f"Output: {output.keys()}\n")
        
        observation = self.render_template(self.config.action_observation_template, output=output)
        return observation

    def parse_action(self, response: dict) -> dict:
        """Parse the action from the message. Returns the action."""
        actions = re.findall(r"```bash\n(.*?)\n```", response["content"], re.DOTALL)
        if len(actions) == 1:
            return {"action": actions[0].strip(), **response}
        raise FormatError(self.render_template(self.config.format_error_template, actions=actions))

    def execute_action(self, action: dict) -> dict:
        try:
            output = self.env.execute(action["action"], cwd=self.sandbox_dir)
        except subprocess.TimeoutExpired as e:
            output = e.output.decode("utf-8", errors="replace") if e.output else ""
            raise ExecutionTimeoutError(
                self.render_template(self.config.timeout_template, action=action, output=output)
            )
        except TimeoutError:
            raise ExecutionTimeoutError(self.render_template(self.config.timeout_template, action=action, output=""))
        self.has_finished(output)
        return output

    def has_finished(self, output: dict[str, str]):
        """Raises Submitted exception with final output if the agent has finished its task."""
        lines = output.get("output", "").lstrip().splitlines()

        if lines and lines[0].strip() == "COMPLETE_TASK_AND_SUBMIT_FINAL_OUTPUT":
            raise Submitted("\n".join(lines[1:]))


def test_cli_agent():
    model_name = "claude-sonnet-4-20250514"
    agent = MyCLIAgent(
        AnthropicModel(model_name=model_name),
        LocalEnvironment(),
        sandbox_dir="./eval_output/test_agent_claude-4/p1/sandbox/",
        log_path="./eval_output/test_agent_claude-4/p1/test_agent_output.log",
        checkpoint_path="./eval_output/test_agent_claude-4/p1/test_agent_checkpoint.json",
    )
    task = "Please write an simple python sort function. You are inside an sandbox directory. You can do anything you want in this sandbox, e.g. add Python files, add tests, etc. You must not touch any other files outside this folder. You need to make sure after finishing the task, the folder contains `./sandbox(where you are now)/main.py`."
    agent.run(task)

if __name__ == "__main__":
    test_cli_agent()