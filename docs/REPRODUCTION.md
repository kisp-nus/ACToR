# Reproduction Guide

The total cost of running all the experiments exceeds $4000 USD and 25–40 person-hours in total. We provide a detailed document for those who are interested in our experiments. We recommend reading the Quick Start section in the [README](README.md) to understand the basic usage of ACToR before reproduction.

## Table of Contents

- [Settings](#settings)
  - [Benchmarks](#benchmarks)
  - [Metrics](#metrics)
- [Overview of Experiments](#overview-of-experiments)
- [Experiments](#experiments)
  - [Experiment 1: Correctness & Adaptability](#experiment-1-correctness--adaptability)
  - [Experiment 2: Ablation Study on Design](#experiment-2-ablation-study-on-design)
  - [Experiment 3: Equal Cost Comparison](#experiment-3-equal-cost-comparison)
  - [Experiment 4: Ablation Study on Configurations](#experiment-4-ablation-study-on-configurations)
  - [Experiment 5: Stability of Results](#experiment-5-stability-of-results)
  - [Experiment 6: Macro Experiment](#experiment-6-macro-experiment)

## Settings

### Benchmarks
We use two benchmarks in our evaluation:
- Micro benchmark: contains 6 programs, located in `./projects_input` directory. Please ignore the "example" project.
- Macro benchmark: contains 57 BSDCore Utilities, located in `./projects_input_BSD` directory.

When running experiments on the micro benchmark, we recommend using the following configuration to launch ACToR:
```json
{
  "max_parallel": 10,
  "input_directory": "projects_input",
  "working_directory": ".working",
  "backups_directory": ".backups",
  "output_directory": "projects_output"
}
```

When running experiments on the macro benchmark, we recommend the following configuration:
```json
{
  "max_parallel": 10,
  "input_directory": "projects_input_BSD",
  "working_directory": ".working_BSD",
  "backups_directory": ".backups_BSD",
  "output_directory": "projects_output_BSD"
}
```
`max_parallel` can be 5–10, depending on your API resources.


### Metrics
As explained in our paper, the experiments are measured using two metrics: absolute pass rate and relative pass rate. The absolute pass rate is the pass rate on the validation test set prepared by us, applicable only to the micro benchmark. The relative pass rate is the pass rate when cross-comparing among different translation methods. We mark them as `absolute` and `relative` when listing experiments.

<!-- ### Collect Costs: -->


## Overview of Experiments

We evaluate ACToR through 6 experiments, each corresponding to a section in our paper:

| # | Experiment | Paper Section | Benchmark | Metric |
|---|------------|---------------|-----------|--------|
| 1 | Correctness & Adaptability | Section 4.2 | Micro | Absolute |
| 2 | Ablation Study on Design | Section 4.2 | Micro | Relative |
| 3 | Equal Cost Comparison | Section 4.2 | Micro | Relative |
| 4 | Ablation Study on Configurations | Section 4.2 | Micro | Absolute |
| 5 | Stability of Results | Section 4.2 | Micro | Absolute |
| 6 | Macro Experiment | Section 4.3 | Macro | Relative |

Each experiment consists of two steps:
1. **Run Translations**: Execute ACToR with specific translator-discriminator settings on all programs in the benchmark.
2. **Collect Statistics**: Run evaluation scripts to compute pass rates, costs, and optionally generate figures.

Please refer to [MODULES](MODULES.md) for the naming conventions used for translators and discriminators. In the tables below, **each row represents a trial that runs ACToR with one translator-discriminator setting on all programs in that benchmark.**

**Tip:** After you get the translation result, you need to specify the instance used in the comparison. For instance, Assume you have 3 instances that all have the same settings, and all finish 10 iterations, for the same program. Which one do you want to use in the comparison? You need to specify the instance id. The evaluation scripts include a helper function `collect_result_instance` that automatically collects corresponding instances using `translator+discriminator+iteration` as a key. This works for most cases, but may require manual specifying the instances for experiments if identical settings exists (e.g., Experiment 5, which has 3 trials based on the same setting).


## Experiments

### Experiment 1: Correctness & Adaptability

**Paper Section:** 4.2 | **Benchmark:** Micro | **Metric:** Absolute

#### Step 1: Run Translations

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| Default ACToR | CC-Sonnet-4 | CC-Sonnet-4-ACToR | 10 |
| Default ACToR | SWE-Sonnet-4.5 | SWE-Sonnet-4.5-ACToR | 10 |
| Default ACToR | SWE-Sonnet-4 | SWE-Sonnet-4-ACToR | 10 |
| Default ACToR | SWE-GPT-5mini | SWE-GPT-5mini-ACToR | 10 |

#### Step 2: Collect Statistics

Run the following script to collect the absolute pass rate and cost:

```bash
python ./evaluation_scripts/eval1.py
```

Then, run the following script to draw the bar figure (optional):

```bash
python ./evaluation_scripts/draw_eval1.py
```

---

### Experiment 2: Ablation Study on Design

**Paper Section:** 4.2 | **Benchmark:** Micro | **Metric:** Relative

#### Step 1: Run Translations

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| ACToR noFuzz | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR-noFuzz | 10 |
| Coverage Baseline | CC-Sonnet-4.5 | CC-Sonnet-4.5-Coverage | 10 |

#### Step 2: Collect Statistics

Run the following script to collect the relative pass rate and cost:

```bash
python ./evaluation_scripts/eval2.py
```

Then, run the following script to draw the cross-comparison heat graph (optional):

```bash
python ./evaluation_scripts/draw_eval2.py
```

---

### Experiment 3: Equal Cost Comparison

**Paper Section:** 4.2 | **Benchmark:** Micro | **Metric:** Relative

#### Step 1: Run Translations

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| Coverage Baseline | CC-Sonnet-4.5 | CC-Sonnet-4.5-Coverage | 25 |

#### Step 2: Collect Statistics

Run the following script to collect the relative pass rate and cost:

```bash
python ./evaluation_scripts/eval3.py
```

This experiment does not have a figure.

---

### Experiment 4: Ablation Study on Configurations

**Paper Section:** 4.2 | **Benchmark:** Micro | **Metric:** Absolute

#### Step 1: Run Translations

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 20 |
| ACToR @ 15 initial tests, 1 new test per iteration | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR-15_1 | 10 |
| ACToR @ 15 initial tests, 5 new tests per iteration | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR-15_5 | 10 |
| *ACToR @ 1 initial test, 3 new tests per iteration | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR-1_3 | 10 |

\*For the trial that starts with only 1 seed test, you need one extra step to prepare the inputs: you need to ensure only one seed test is available in the input folder of each project. We provide a script for this preparation:

```bash
python ./evaluation_scripts/prepare_single_seed.py 
```

This will back up the `./projects_input/project_name/tests00.jsonl` file and remove all lines except the first line.

After this trial, **do not forget to revert it back** before continuing with other experiments.

```bash
python ./evaluation_scripts/revert_back_seed.py  
```

#### Step 2: Collect Statistics

Run the following script to collect the absolute pass rate and cost:

```bash
python ./evaluation_scripts/eval4.py
```

Then, run the following script to draw the bar figure (optional):

```bash
python ./evaluation_scripts/draw_eval4.py
```

---

### Experiment 5: Stability of Results

**Paper Section:** 4.2 | **Benchmark:** Micro | **Metric:** Absolute

#### Step 1: Run Translations

Run default ACToR 3 times.

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| #1 Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| #2 Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| #3 Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |

> **Note:** Since the 3 trials have the exact same setting, the `collect_result_instance` helper function cannot distinguish them automatically. You may need to manually specify the instances participating in the computation.

#### Step 2: Collect Statistics

Run the following script to collect the absolute pass rate standard deviation:

```bash
python ./evaluation_scripts/eval5.py
```

This experiment does not have a figure.

---

### Experiment 6: Macro Experiment

**Paper Section:** 4.3 | **Benchmark:** Macro | **Metric:** Relative

#### Step 1: Run Translations

| Trial | Translator | Discriminator | #Iter |
|--------|------------|---------------|-------|
| Default ACToR | CC-Sonnet-4.5 | CC-Sonnet-4.5-ACToR | 10 |
| Coverage Baseline | CC-Sonnet-4.5 | CC-Sonnet-4.5-Coverage | 10 |

#### Step 2: Collect Statistics

Run the following script to collect the relative pass rate and cost:

```bash
python ./evaluation_scripts/eval6.py
```

Then, run the following script to draw the final comparison figure (optional):

```bash
python ./evaluation_scripts/draw_eval6.py
```

