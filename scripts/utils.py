import os
import json
import subprocess
import re
import argparse
import copy
import glob
import openai
import datetime
import shutil
import time
import sys
import benchmarks

SCRIPT_NAME = "r.sh"
TOKEN_COUNTER_PATH = "/tmp/gpt_reduction/tools/token_counter_deploy.jar"
PERSES_PATH = "/tmp/gpt_reduction/tools/perses_deploy.jar"
VULCAN_PATH = "/tmp/gpt_reduction/tools/vulcan_deploy.jar"
LOG_FILE_NAME = None
LANGUAGE = None
PROGRAM_NAME = None
ROOT_FOLDER = None

def execute_cmd(cmd, output=False):
    if output:
        process = subprocess.run(f"{cmd}", shell=True, check=False)
    else:
        process = subprocess.run(f"{cmd}", shell=True, stdout=subprocess.DEVNULL,
                                 stderr=subprocess.DEVNULL, check=False)
    return process.returncode

def property_test(rerun=False):
    result_file_name = "property_test_result"
    result_file_path = os.path.join("./", result_file_name)
    if not rerun:
        if (os.path.exists(result_file_path)):
            saved_result = load_file(result_file_path)
            if (saved_result == "fail"):
                return False
            else:
                return True
    
    ret = execute_cmd("./r.sh")
    if ret == 1:
        save_file("./", result_file_name, "fail")
        return False
    
    save_file("./", result_file_name, "pass")
    return True

def save_file(folder_path, file_name, content):
    os.makedirs(folder_path, exist_ok=True)
    file_path = os.path.join(folder_path, file_name)
    with open(file_path, "w") as file_writer:
        file_writer.write(content)

def save_program_file(folder_path, program):
    os.makedirs(folder_path, exist_ok=True)
    file_path = os.path.join(folder_path, PROGRAM_NAME)
    with open(file_path, "w") as file_writer:
        file_writer.write(program)


def save_json_file(folder_path, file_name, json_object):
    os.makedirs(folder_path, exist_ok=True)
    file_path = os.path.join(folder_path, file_name)
    with open(file_path, "w") as file_writer:
        json.dump(json_object, file_writer)


def load_file(path):
    with open(path, "r") as file_reader:
        program = file_reader.read()
    return program


def load_json_file(path):
    with open(path, "r") as file_reader:
        json_object = json.load(file_reader)
    return json_object

def check_finish(folder):
    status_file = os.path.join(folder, "finish")
    if os.path.exists(status_file):
        return True
    return False

def compute_md5(file_path):
    output = subprocess.check_output(
        f"md5sum {file_path}", shell=True)
    md5_str_line = output.decode().splitlines()[-1]
    md5_str = md5_str_line.split(" ")[0]
    return md5_str

def count_token(program_path):
    current_md5 = compute_md5(program_path)

    program_folder = os.path.dirname(program_path)
    program_size_file_name = "program_size.json"
    program_size_path = os.path.join(program_folder, program_size_file_name)
    if (os.path.exists(program_size_path)):
        size_json = load_json_file(program_size_path)
        if (current_md5 == size_json["md5"]):
            return size_json["size"]
        
    output = subprocess.check_output(
        f"java -jar {TOKEN_COUNTER_PATH} -- {program_path}", shell=True)
    size_str = output.decode().splitlines()[-1]
    size_int = int(size_str)
    json_object = {"md5": current_md5, "size": size_int}
    save_json_file(program_folder, program_size_file_name, json_object)
    return int(size_str)


def extract_code(text):
    pattern = r"```[\S]*(.*?)```"
    result = re.findall(pattern, text, re.DOTALL)
    if result:
        return result[-1]
    return text


def extract_json(text):
    pattern = r"```(?:json|JSON|Json)(.*?)```"
    result = re.findall(pattern, text, re.DOTALL)
    if result:
        json_string = result[-1]
        # in case gpt generates some \\\n and make json parsing fail
        json_string = json_string.replace("\\\n", "")
        try:
            return json.loads(json_string)
        except json.JSONDecodeError:
            return {}
    else:
        return {}

def get_current_version():
    result = subprocess.run("git rev-parse --short HEAD", stdout=subprocess.PIPE, shell=True, text=True, check=False)
    version = result.stdout.strip()
    return version

def get_args_string(parser):
    """Generate a string representation of the arguments provided explicitly by the user."""
    args = parser.parse_args()
    args_dict = copy.deepcopy(vars(args))
    default_args = {action.dest: action.default for action in parser._actions}

    # Keep only user-provided arguments (non-default)
    user_args = {key: value for key, value in args_dict.items() if default_args.get(key, None) != value}

    # Generate the string representation
    args_string = '_'.join(f'{key}_{value}' for key, value in user_args.items())
    script_name = sys.argv[0]
    args_string = script_name + "_" + args_string
    pattern = re.compile(r'[^a-zA-Z0-9]+')
    return pattern.sub('_', args_string)

def initialize_parser():
    parser = argparse.ArgumentParser(description="Process some inputs.")
    parser.add_argument("--prompts", type=str, required=False, default="/tmp/gpt_reduction/prompts/prompts.json", help="Configuration file about prompts")
    parser.add_argument("--benchmark-suite", type=str, required=True, help="Folder of benchmark suite")
    parser.add_argument("--case", type=str, required=False, default=None, help="Benchmark ID")
    parser.add_argument("--trial", type=int, required=False, default=5, help="Number of trials in LLMs")
    parser.add_argument("--temperature", type=float, required=False, default=1.0, help="Temperature used in LLMs")
    parser.add_argument("--max-jobs", type=int, required=False, default=1, help="The maximum number of concurrent tasks allowed")
    parser.add_argument("--llm-version", type=str, required=False, default="gpt-3.5-turbo-0613", help="LLM version")
    parser.add_argument("--disable-multi-level", action="store_true", required=False, default=False, help="Disable multi-level prompt")
    parser.add_argument("--id", type=str, required=False, help="A unique identifier used to differentiate each rerun")
    return parser

def init_openai_api_key():
    """Initialize OpenAI API key or raise an error if it is not set."""
    openai_api_key = os.getenv("OPENAI_API_KEY")
    if not openai_api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set in the environment variables.")
    openai.api_key = openai_api_key

def init_root_folder(folder):
    global ROOT_FOLDER
    ROOT_FOLDER = folder

def init_language(folder):
    """Determine the programming language and initialize LANGUAGE and PROGRAM_NAME."""
    global LANGUAGE, PROGRAM_NAME
    program_name_dict = {
        "c": "small.c",
        "cpp": "small.cpp",
        "rs": "small.rs",
        "go": "small.go",
        "js": "small.js"
    }

    for _, _, filenames in os.walk(folder):
        for ext, file_name in program_name_dict.items():
            if file_name in filenames:
                LANGUAGE = ext
                PROGRAM_NAME = file_name
                return
    raise ValueError("Could not determine the programming language.")

def get_benchmarks(benchmark_suite_folder, case):
    case_list = []
    if (case is None):
        for item in os.listdir(benchmark_suite_folder):
            if os.path.isdir(os.path.join(benchmark_suite_folder, item)):
                case_list.append(item)
    else:
        case_list.append(case)
    
    case_list = sorted(case_list)
    return case_list

def print_and_log(message, level):
    # make indentation
    indent = ""
    for _ in range(level):
        indent = indent + "  "

    # make time stamp
    now = datetime.datetime.now()
    time_string = now.strftime("%Y-%m-%d %H:%M:%S")
    message = f"{indent}[{time_string}] {message}"
    print(message)
    with open(LOG_FILE_NAME, 'a') as file:
        file.write(message + '\n')

def call_perses(output_folder, level):
    print_and_log(f"start perses", level=level)
    output_program_path = os.path.join(output_folder, PROGRAM_NAME)
    output_script_path = os.path.join(output_folder, SCRIPT_NAME)
    working_folder = os.path.join(output_folder, "perses")
    working_program_path = os.path.join(working_folder, PROGRAM_NAME)
    working_script_path = os.path.join(working_folder, SCRIPT_NAME)
    working_log_path = os.path.join(working_folder, "perses_log.txt")
    os.makedirs(working_folder, exist_ok=True)

    if not check_finish(working_folder):
        start_time = time.time()
        shutil.copy(output_program_path, working_program_path)
        shutil.copy(output_script_path, working_script_path)

        execute_cmd(
            f"java -jar {PERSES_PATH} --input {working_program_path} --test {working_script_path} --output-dir {working_folder} --query-caching false --threads 1 > {working_log_path} 2>&1")

        call_formatter(working_folder)
        end_time = time.time()
        save_file(working_folder, "finish", f"{end_time-start_time}")
    shutil.copy(working_program_path, output_program_path)

    program_size = count_token(output_program_path)
    print_and_log(f"Finish perses: {program_size} tokens", level=level)

def call_vulcan(output_folder, level):
    print_and_log(f"start vulcan", level=level)
    output_program_path = os.path.join(output_folder, PROGRAM_NAME)
    output_script_path = os.path.join(output_folder, SCRIPT_NAME)
    working_folder = os.path.join(output_folder, "vulcan")
    working_program_path = os.path.join(working_folder, PROGRAM_NAME)
    working_script_path = os.path.join(working_folder, SCRIPT_NAME)
    working_log_path = os.path.join(working_folder, "vulcan_log.txt")
    os.makedirs(working_folder, exist_ok=True)

    if not check_finish(working_folder):
        start_time = time.time()
        shutil.copy(output_program_path, working_program_path)
        shutil.copy(output_script_path, working_script_path)

        execute_cmd(
            f"java -jar {VULCAN_PATH} --input {working_program_path} --test {working_script_path} --output-dir {working_folder} --enable-vulcan true --query-caching false --threads 1 > {working_log_path} 2>&1"
        )

        call_formatter(working_folder)
        end_time = time.time()
        save_file(working_folder, "finish", f"{end_time-start_time}")
    shutil.copy(working_program_path, output_program_path)

    program_size = count_token(output_program_path)
    print_and_log(f"Finish vulcan: {program_size} tokens", level=level)

def call_creduce(output_folder, level):
    print_and_log(f"start creduce", level=level)
    output_program_path = os.path.join(output_folder, PROGRAM_NAME)
    output_script_path = os.path.join(output_folder, SCRIPT_NAME)
    working_folder = os.path.join(output_folder, "creduce")
    working_program_path = os.path.join(working_folder, PROGRAM_NAME)
    working_script_path = os.path.join(working_folder, SCRIPT_NAME)
    working_log_path = os.path.join(working_folder, "creduce_log.txt")
    os.makedirs(working_folder, exist_ok=True)

    if not check_finish(working_folder):
        start_time = time.time()
        shutil.copy(output_program_path, working_program_path)
        shutil.copy(output_script_path, working_script_path)

        execute_cmd(
            f" creduce --n 1 --timing --no-cache {working_script_path} {working_program_path} > {working_log_path} 2>&1"
        )

        call_formatter(working_folder)
        end_time = time.time()
        save_file(working_folder, "finish", f"{end_time-start_time}")
    shutil.copy(working_program_path, output_program_path)

    program_size = count_token(output_program_path)
    print_and_log(f"Finish creduce: {program_size} tokens", level=level)

def call_formatter(working_folder):
    working_program_path = os.path.join(working_folder, PROGRAM_NAME)
    working_script_path = os.path.join(working_folder, SCRIPT_NAME)

    format_folder = os.path.join(working_folder, "format")
    os.makedirs(format_folder, exist_ok=True)
    
    shutil.copy(working_program_path, format_folder)
    shutil.copy(working_script_path, format_folder)

    current_path = os.getcwd()
    os.chdir(format_folder)
    if LANGUAGE in ("c", "cpp"):
        tmp_file = "tmp"
        execute_cmd(f"clang-format {PROGRAM_NAME} > {tmp_file}", output=True)
        if property_test():
            shutil.copy(tmp_file, working_program_path)
    elif LANGUAGE in ("rs",):
        execute_cmd(f"rustfmt {PROGRAM_NAME}", output=True)
        if property_test():
            shutil.copy(PROGRAM_NAME, working_program_path)
    elif LANGUAGE in ("go",):
        tmp_file = "tmp"
        execute_cmd(f"gofmt {PROGRAM_NAME} > {tmp_file}", output=True)
        if property_test():
            shutil.copy(tmp_file, working_program_path)
    elif LANGUAGE in ("js",):
        tmp_file = "tmp"
        execute_cmd(f"js-beautify {PROGRAM_NAME} > {tmp_file}", output=True)
        if property_test():
            shutil.copy(tmp_file, working_program_path)
    else:
        pass
    os.chdir(current_path)

# Function to determine the benchmark suite based on folder names in RESULT_PATH
def determine_benchmark_suite(result_path):
    for entry in os.listdir(result_path):
        full_path = os.path.join(result_path, entry)
        if os.path.isdir(full_path):
            if 'gcc' in entry or 'clang' in entry:
                return benchmarks.benchmark_suite_c
            elif 'rust' in entry:
                return benchmarks.benchmark_suite_rust
            elif 'js' in entry:
                return benchmarks.benchmark_suite_js
    return None  # Fallback in case no matching folder is found