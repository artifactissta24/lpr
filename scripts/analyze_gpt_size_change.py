import os
import sys
import csv
import utils
import shutil

def extract_operations(case_path):
    operations = set()
    for root, dirs, files in os.walk(case_path):
        for dir in dirs:
            if dir.startswith("operation_"):
                operation = dir.replace("operation_", "")
                operations.add(operation)
    return sorted(operations)

def compute_size_changes_of_operation(case_path, operation):
    total_diff = 0

    # Recursively search for all folders named operation_{operation}
    for root, dirs, files in os.walk(case_path):
        for dir in dirs:
            if dir == f"operation_{operation}":
                operation_folder = os.path.join(root, dir)
                program_file = os.path.join(operation_folder, operation, utils.PROGRAM_NAME)
                orig_file = os.path.join(operation_folder, operation, utils.PROGRAM_NAME + ".orig")
                
                # Ensure both files exist
                if os.path.exists(program_file) and os.path.exists(orig_file):
                    program_count = utils.count_token(program_file)

                    # Copy and rename .orig file
                    tmp_folder = os.path.join("/tmp", operation)
                    os.makedirs(tmp_folder, exist_ok=True)
                    tmp_orig_file = os.path.join(tmp_folder, utils.PROGRAM_NAME)
                    shutil.copy(orig_file, tmp_orig_file)
                    
                    orig_count = utils.count_token(tmp_orig_file)
                    diff = program_count - orig_count
                    total_diff += diff

                    # Clean up the temporary folder
                    shutil.rmtree(tmp_folder)

    return total_diff


def compute_size_changes_of_operation_after_perses(case_path, operation):
    total_diff = 0

    # Recursively search for all folders named operation_{operation}
    for root, dirs, files in os.walk(case_path):
        for dir in dirs:
            if dir == f"operation_{operation}":
                operation_folder = os.path.join(root, dir)
                program_file = os.path.join(operation_folder, "perses", utils.PROGRAM_NAME)
                orig_file = os.path.join(operation_folder, operation, utils.PROGRAM_NAME + ".orig")
                
                # Ensure both files exist
                if os.path.exists(program_file) and os.path.exists(orig_file):
                    program_count = utils.count_token(program_file)

                    # Copy and rename .orig file
                    tmp_folder = os.path.join("/tmp", operation)
                    os.makedirs(tmp_folder, exist_ok=True)
                    tmp_orig_file = os.path.join(tmp_folder, utils.PROGRAM_NAME)
                    shutil.copy(orig_file, tmp_orig_file)
                    
                    orig_count = utils.count_token(tmp_orig_file)
                    diff = program_count - orig_count
                    total_diff += diff

                    # Clean up the temporary folder
                    shutil.rmtree(tmp_folder)

    return total_diff


def main():
    RESULT_PATH = sys.argv[1]
    benchmark_suite = utils.determine_benchmark_suite(RESULT_PATH)
    utils.init_language(RESULT_PATH)

    with open(os.path.join(RESULT_PATH, 'size_change_before_perses.csv'), 'w', newline='') as csvfile_before_perses, \
    open(os.path.join(RESULT_PATH, 'size_change_after_perses.csv'), 'w', newline='') as csvfile_after_perses:
        csv_writer_before_perses = csv.writer(csvfile_before_perses)
        csv_writer_after_perses = csv.writer(csvfile_after_perses)

        # First pass to extract and add operations to header
        all_operations = set()
        for target in benchmark_suite:
            case_path = os.path.join(RESULT_PATH, target)
            operations = extract_operations(case_path)
            all_operations.update(operations)
        
        header = ["target"] + sorted(all_operations)
        csv_writer_before_perses.writerow(header)
        csv_writer_after_perses.writerow(header)
        print(header)

        # Second pass to process each target
        for target in benchmark_suite:
            row_before_perses = [target]
            row_after_perses = [target]
            case_path = os.path.join(RESULT_PATH, target)
            
            if not os.path.exists(case_path):
                row_before_perses.extend([None] * len(all_operations))
                row_after_perses.extend([None] * len(all_operations))
                csv_writer_before_perses.writerow(row_before_perses)
                csv_writer_after_perses.writerow(row_before_perses)
                print(f"{target}: not available")
                continue

            for operation in sorted(all_operations):
                diff_before_perses = compute_size_changes_of_operation(case_path, operation)
                diff_after_perses = compute_size_changes_of_operation_after_perses(case_path, operation)
                row_before_perses.append(diff_before_perses)
                row_after_perses.append(diff_after_perses)
            
            csv_writer_before_perses.writerow(row_before_perses)
            csv_writer_after_perses.writerow(row_after_perses)

            print(f"before perses: {row_before_perses}")
            print(f"after perses: {row_after_perses}")

if __name__ == "__main__":
    main()

