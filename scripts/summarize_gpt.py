import os
import re
import sys
import csv
import utils

# Function to calculate total time from 'finish' files
def calculate_total_time(path):
    total_time = 0.0
    for root, dirs, files in os.walk(path):
        for file in files:
            if file == 'finish':
                with open(os.path.join(root, file), 'r') as f:
                    try:
                        total_time += float(f.read().strip())
                    except ValueError:
                        pass  # Ignore files that do not contain a valid float
    return int(total_time)

# Updated function to calculate GPT time and query count
def calculate_gpt_info(path):
    gpt_time = 0.0
    gpt_query_count = 0
    for root, dirs, files in os.walk(path):
        # Exclude 'perses' folders
        if 'perses' != os.path.basename(root):
            for file in files:
                if file == 'finish':
                    with open(os.path.join(root, file), 'r') as f:
                        try:
                            gpt_time += float(f.read().strip())
                            gpt_query_count += 1  # Increment query count for each 'finish' file
                        except ValueError:
                            pass  # Ignore non-float files
    return int(gpt_time), gpt_query_count

# Function to get token number from log.txt
def get_token_num_from_log(file):
    try:
        with open(file, 'r') as f:
            lines = f.readlines()
        last_line = lines[-1] if lines else ''
        match = re.search(r'reduction ratio:\s*(\d+)/\d+', last_line)
        return match.group(1) if match else None
    except FileNotFoundError:
        return None

# Function to count the number of 'trial' folders and sum the execution counts from perses_log.txt
def get_query_number(case_path):
    trial_count = 0
    perses_execution_count_sum = 0

    for root, dirs, files in os.walk(case_path):
        # Count trial folders
        trial_count += len([d for d in dirs if 'trial' in d])
        # Sum execution counts from perses_log.txt
        for file in files:
            if file == 'perses_log.txt':
                with open(os.path.join(root, file), 'r') as f:
                    last_line = f.readlines()[-1]
                    match = re.search(r'Test script execution count: (\d+)', last_line)
                    if match:
                        perses_execution_count_sum += int(match.group(1))

    return trial_count + perses_execution_count_sum

# Function to count the number of folders containing 'iteration'
def get_iteration_count(case_path):
    iteration_count = 0
    for root, dirs, files in os.walk(case_path):
        iteration_count += len([d for d in dirs if 'iteration' in d])
    return iteration_count

RESULT_PATH = sys.argv[1]
benchmark_suite = utils.determine_benchmark_suite(RESULT_PATH)

with open(os.path.join(RESULT_PATH, 'summary.csv'), 'w', newline='') as csvfile:
    csv_writer = csv.writer(csvfile)
    csv_writer.writerow(["target", "token num", "time", "query number", "iteration", "gpt time", "gpt query"])

    for target in benchmark_suite:
        row = [target]  # Initialize row with target as the first column
        case_path = os.path.join(RESULT_PATH, target)

        if not os.path.exists(case_path):
            print(f"{target} is not available")
            csv_writer.writerow([target, None, None, None, None, None, None])  # Write only target if not available
            continue

        # Get token number
        log_file = os.path.join(case_path, "log.txt")
        token_num = get_token_num_from_log(log_file)

        # Calculate time
        time = calculate_total_time(case_path)

        # Calculate query number
        query_number = get_query_number(case_path)

        # Calculate iteration count
        iteration_count = get_iteration_count(case_path)

        # Calculate GPT time and query count
        gpt_time, gpt_query_count = calculate_gpt_info(case_path)

        if token_num is None:
            print(f"{target}: token number not available")
            csv_writer.writerow([target, None, None, None, None, None, None])
        else:
            print(f"target: {target}: token num: {token_num}, time: {time}, query number: {query_number}, iteration: {iteration_count}, gpt time: {gpt_time}, gpt query: {gpt_query_count}")
            row.extend([token_num, time, query_number, iteration_count, gpt_time, gpt_query_count])
            csv_writer.writerow(row)
