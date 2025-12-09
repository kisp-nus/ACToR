# Translator and Discriminator Guide

This document provides an overview of the **Translator** and **Discriminator** modules in ACToR, explains how they interact with the main system, and outlines how to design your own modules.

## Overview of ACToR

ACToR is built around an **adversarial collaboration** between two components:

- Translator: Generates or refines Rust code according to the provided specifications (e.g., make the translated code pass the test cases).

- Discriminator: Detects mismatches in the translated code and produces new specifications (e.g., new tests) for the next iteration.

The main ACToR system, implemented in `./scripts/ACToR.py`, coordinates iterative refinement by repeatedly invoking these two modules.


---

Each module is invoked in a **one-shot** fashion:

- ACToR calls the translator function → the translator performs its work → returns.

- ACToR calls the discriminator function → the discriminator performs its work → returns.

There is **no further interaction** with the main system during each call. The sub-task for that iteration is considered complete once the function returns.

Each module is **fully independent**. They can be placed in its own Python file, can use its own prompting strategy and internal workflow. They only needs to implement the correct interface for ACToR to call. This makes it easy to swap modules in and out or create entirely custom implementations.

Advantages of This Architecture

- High Flexibility: Translators and discriminators may require very different structures. The loose coupling between modules and the main system gives users maximum freedom to design custom workflows.

- Simple Integration: To integrate your own module, you only need to provide a function matching the expected interface. There are no additional constraints—the function can implement any internal logic.

Limitations

- Limited Fine-Grained Control :The ACToR main system cannot observe internal progress within a translation/discrimination step—it only knows when the step starts and ends.

    *To mitigate this, in addition to checking logs, we use the auxiliary tool `lproc` to provide finer control and insight into long-running sub-processes. Please read the following doc for guidance*

🚧 This document and the architecture are still under active development.
We are exploring designs that make it easier for new users to get started and support more complex translation pipelines involving multiple agents.

If you have suggestions or discover issues in the current implementation, please reach out—we welcome feedback.


## Translator

The goal of the translator is to produce/improve the translated code at each iteration, based on the new specifications (in our default implementation, the specifications are test cases).

### Available Translators

| Name | Model & Agent | Description |
|------|---------------|-------------|
| CC-Sonnet-4.5 | Claude-Sonnet-4.5 & Claude Code | The default translator used in our experiments |
| SWE-Sonnet-4.5 | Claude-Sonnet-4.5 & Mini-SWE-Agent | Translator with open-source SWE agent |
| SWE-Sonnet-4 | Claude-Sonnet-4 & Mini-SWE-Agent | SWE agent with Sonnet-4 model |
| SWE-GPT-5mini | GPT-5mini & Mini-SWE-Agent | SWE agent with GPT-5mini model |

### How It Works

The interaction between the translator and ACToR main loop is simple: the translator is treated as a "pure" function. It takes in the necessary information about the task, conducts translation in the assigned sandbox, and returns when the translation is finished.

### Using Your Own Translator

You can use your own translator freely. You only need to provide a single `translator` function in the `actor.py` main file, and make it match the input signature of the other `translator` functions. The detailed API references will be provided in the documentation.

Inside the translator function, you can do anything you want. But we recommend the translator to at least have the following two components:

1. **Main translator**: Run the agents, LLMs, etc., with translation prompts.
2. **Translation Validator**: A heuristic that checks whether the specifications (test cases) are satisfied by the produced/improved code. If not, retry the task.

You can check our implementation and prompts for the default `CC` and `SWE` translators as a reference.

---

## Discriminator

The goal of the discriminator is to discover mismatches between the translated code and the input source code, then provide new feedback/specifications for the translator to improve. In our implementation, the feedback consists of new test cases that reveal bugs in the translated code.

### Available Discriminators

| Name | Model & Agent | Description |
|------|---------------|-------------|
| CC-Sonnet-4.5-ACToR | Claude-Sonnet-4.5 & Claude Code | Default discriminator (15 initial tests, 3 new tests/iter) |
| CC-Sonnet-4.5-ACToR-1_3 | Claude-Sonnet-4.5 & Claude Code | Ablation: 1 initial test, 3 new tests/iter |
| CC-Sonnet-4.5-ACToR-15_1 | Claude-Sonnet-4.5 & Claude Code | Ablation: 15 initial tests, 1 new test/iter |
| CC-Sonnet-4.5-ACToR-15_5 | Claude-Sonnet-4.5 & Claude Code | Ablation: 15 initial tests, 5 new tests/iter |
| CC-Sonnet-4.5-ACToR-noFuzz | Claude-Sonnet-4.5 & Claude Code | Without fuzzing script, used in ablation study |
| CC-Sonnet-4.5-Coverage | Claude-Sonnet-4.5 & Claude Code | Coverage-based discriminator |
| SWE-Sonnet-4.5-ACToR | Claude-Sonnet-4.5 & Mini-SWE-Agent | SWE agent discriminator |
| SWE-Sonnet-4-ACToR | Claude-Sonnet-4 & Mini-SWE-Agent | SWE agent with Sonnet-4 model |
| SWE-GPT-5mini-ACToR | GPT-5mini & Mini-SWE-Agent | SWE agent with GPT-5mini model |

### Using Your Own Discriminator

You can design your own discriminator easily. Similar to `translator`, the discrimination process is treated as a "pure function". You only need to provide a `discriminator` function in the `actor.py` main file, and make it match the input signature of the other `discriminator` functions.

Inside the discriminator function, you can include any functionality you want (even an empty discriminator that does nothing). We recommend the discriminator to at least have the following two components:

1. **Main discriminator**: Run the agents, LLMs, etc., with discrimination prompts to discover mismatches and add new test cases.
2. **Tests Validator**: A heuristic that checks whether the specifications (test cases) are valid, i.e., the source C code has correct behavior on these tests. If not, retry the task.

You can check our implementation and prompts for the default `CC-Sonnet-4.5-ACToR.py` etc. as a reference.



## Designing Custom Specifications

### Specification Contract

The translator and discriminator should work as a pair: the specifications should be a contract agreed upon between the translator and discriminator.

In our default implementation, the specifications are test cases:
- The **discriminator** discovers and adds new test cases
- The **translator** improves the translation to pass the new test cases

If you use different specifications, you should explain the format of specifications to both the translator and discriminator in the prompt, and use these two as a pair. **Mismatch in specification formats will result in failure.**

### Validation

When you design a specification, you need to consider how to validate if the specifications are satisfied. In our case, this is simple: we just run the translated code on the tests and see if all tests pass.

The validator can be:
- Heuristics (rule-based checks)
- Another LLM agent
- Formal verification tools
- Any combination of the above

---

## (WIP) Guide for Lproc & Sand

In the default translators and discriminators provided with ACToR, we use the lproc and sand tools to manage **Claude Code** instances during translation.

#### `lproc`: Managing Long-Running Processes

`lproc` is used to register and control long-running processes. In ACToR, it is responsible for launching and managing Claude Code in headless mode for code-generation tasks. For more details, see the documentation in `__utils/_lproc/README.md`.

In addition to lproc, there is a visualization tool called hlproc (run it by typing `hlproc` in the command line), which provides a user-friendly interface for monitoring and managing running instances.

#### `sand`: Controlled Workspace Assignment

`sand` is used to explicitly assign a working directory to an process and manage its privileges for modifying files. In our implementation, we use this to ensure that Claude Code cannot modify files outside the assigned workspace. More details can be found in `__utils/_sand/README.md`.


## (WIP) API Reference
<!-- 
### Translator Function Signature

```python
def translator(
    sandbox_dir: str,
    iteration: int,
    task_info: dict,
    **kwargs
) -> dict:
    """
    Perform translation in the sandbox directory.
    
    Args:
        sandbox_dir: Path to the sandbox working directory
        iteration: Current iteration number
        task_info: Dictionary containing task metadata
        
    Returns:
        dict with keys:
            - 'status': 'success' or 'error'
            - 'message': Description of what was done
            - 'files_modified': List of modified file paths
    """
    pass
```

### Discriminator Function Signature

```python
def discriminator(
    sandbox_dir: str,
    iteration: int,
    task_info: dict,
    **kwargs
) -> dict:
    """
    Discover mismatches and add new test cases.
    
    Args:
        sandbox_dir: Path to the sandbox working directory
        iteration: Current iteration number
        task_info: Dictionary containing task metadata
        
    Returns:
        dict with keys:
            - 'status': 'passed' or 'failed'
            - 'new_tests': Number of new tests added
            - 'bugs_found': List of discovered bugs
    """
    pass
``` -->


## 🚧 Coming Soon

We're actively working on improving ACToR! Here's what's on our roadmap:

**Documentation & Tutorials**
- More example implementations for custom translators and discriminators
- Guide for designing own specification formats

**Core Improvements**
- A better input/output format for more fine-grained control of translation tasks
- Plugin system for easier integration of new agents

**Developer Experience**
- Enhanced debugging and logging utilities

💡 *Have suggestions? Feel free to open an issue or PR!*
