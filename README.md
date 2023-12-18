# Artifact for "LPR: Large Language Models-Aided Program Reduction"

## Introduction

Thank you for evaluating this artifact!

To evaluate this artifact, a Linux machine with [docker](https://docs.docker.com/get-docker/) installed is needed.

### List of Claims Supported by the Artifact

- LPR is more effective than Vulcan.

- LPR is on par with Vulcan in terms of efficiency. LPR is more efficient than Vulcan on complex programs.

- LPR generalizes well on C, Rust and JavaScript.

### Notes

- All the experiments take long time to finish, so it is recommended to use tools like screen and tmux to manage sessions if the experiments are run on remote server. We also provide flags for multi-processing.

- The evaluation results of LPR may not exactly the same as shown in the paper, because LLMs have internal randomness. Replicating the experiments for multiple times will mitigate such impact. However, the deviation should be trivial, and the results should still support the original claims in the paper.

- It is possible for remote calls to LLMs to lose connection and raise exceptions, because the resource for user to invoke LLMs might be limited. Therefore, we add logics and scripts for quick restart from the intermediate results.

### Docker Environment Setup

1. If docker is not installed, install it by following the [instructions](https://docs.docker.com/get-docker/).
2. Install the docker image.

   ```bash
   docker pull codesubmission278/lpr:latest
   # This step might takes a while, mainly depending on the network bandwidth. It also takes up much disk space (nearly 80GB)
   ```

3. Start a container.

   ```bash
   docker container run --cap-add SYS_PTRACE --interactive --tty codesubmission278/lpr:latest /bin/bash
   # You should be at /tmp after the above command finishes
   # Your user name should be `coq` and all the following command are executed in docker

   # the root folder of the project is /tmp/gpt_reduction/
   cd /tmp/gpt_reduction/
   ```

### Benchmark Suites

In this project,
benchmark suite are in folder `./benchmark_suites`.

1. `./benchmark_suites/c`: 20 benchmarks in C (Benchmark-C).

2. `./benchmark_suites/rust`: 20 benchmarks in Rust (Benchmark-Rust).

3. `./benchmark_suites/js`: 10 benchmarks in JavaScript (Benchmark-JS).

### Previous Results

Some previous results on Benchmark-C are stored in `./old_results`.

### Reproduce RQ1 & RQ2: the Effectiveness and Efficiency of Perses, Vulcan, C-Reduce and LPR.

1. Run Perses.

```bash
cd /tmp/gpt_reduction/
# run perses on Benchmark-C
python3 scripts/run_perses.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/original/ --max-jobs 8
# "--max_jobs" enables running multiple benchmarks concurrently
# "--benchmark-suite" specifies the benchmark suite to evaluate on
```

Assume the current code version is 1234567 (github --version), then the results will be stored in:

> ./results/1234567/scripts_run_perses_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_original_max_jobs_8/

To get summarized information

```bash
python summarize_perses_or_vulcan.py ./results/1234567/scripts_run_perses_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_original_max_jobs_8/
```

2. Run Vulcan

```bash
# Similar to how we run Perses:
cd /tmp/gpt_reduction/
python3 scripts/run_vulcan.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/original/ --max-jobs 8
```

To get summarized information

```bash
python summarize_perses_or_vulcan.py ./results/1234567/scripts_run_vulcan_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_original_max_jobs_8/
```

3. Run C-Reduce

```bash
# Similar to how we run Perses:
cd /tmp/gpt_reduction/
python3 scripts/run_vulcan.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/original/ --max-jobs 8
```

To get summarized information

```bash
python summarize_creduce.py ./results/1234567/scripts_run_creduce_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_original_max_jobs_8/
```

4. Run LPR
   Before running LPR, we need to add an OPENAI_API_KEY to the environment variable.

```bash
export OPENAI_API_KEY=sk-xxxxxxx
```

```bash
cd /tmp/gpt_reduction/
python scripts/run_gpt_reduction.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/perses_results_rename --id 0
# In case the invocation of LLMs timeouts and terminates the whole process, we can wrap the command line with ./scripts/keep_running.sh, which continues to run the given command until it exits with 0.
./scripts/keep_running.sh "python scripts/run_gpt_reduction.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/perses_results_rename --id 0"
# flag "--id" helps to distinguish the id of runs for data storage. In the next run, if all other flags are identical, use "--id 1" to prevent the dulicated result folder.
```

To get summarized information

```bash
python ./scripts/summarize_gpt.py ./results/1234567/scripts_run_gpt_reduction_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_perses_results_rename_id_0/
```

### Reproduce RQ3: Effectiveness of Each Transformation.

To calculate the contribution of each transformation, run

```bash
python ./scripts/analyze_gpt_size_change.py ./results/1234567/scripts_run_gpt_reduction_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_perses_results_rename_id_0/
```

This script will save the size change of each transformation on each benchmark to csv files in the result folder "./results/1234567/scripts_run_gpt_reduction_py_benchmark_suite_tmp_gpt_reduction_benchmark_suites_c_perses_results_rename_id_0/".

### Other Experiments

To specify another version of GPT instead of the default "gpt-3.5-turbo-0613", use flag "--llm-version", for example:

```bash
python scripts/run_gpt_reduction.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/perses_results_rename --llm-version gpt-3.5-turbo-1106 --id 0
```

To specify the temperature, use flag "--temperature", for example:

```bash
python scripts/run_gpt_reduction.py --benchmark-suite /tmp/gpt_reduction/benchmark_suites/c/perses_results_rename --temperature 0.5 --id 0
```
