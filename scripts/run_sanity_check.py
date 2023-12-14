import os
import utils

def main():

    utils.init_openai_api_key()
    parser = utils.initialize_parser()
    args = parser.parse_args()

    case = args.case
    benchmark_suite_folder = args.benchmark_suite

    utils.init_language(benchmark_suite_folder)
    utils.init_root_folder(os.getcwd())

    # get case list
    case_list = utils.get_benchmarks(benchmark_suite_folder, case)

    for case in case_list:        
        os.chdir(os.path.join(benchmark_suite_folder, case))
        if (utils.property_test(rerun=True)):
            print(f"Sanity check passed on {case}")
        else:
            print(f"Sanity check failed on {case}")
        
if __name__ == "__main__":
    main()