#!/usr/bin/env bash

LOG_FILE=$"Files/log.txt"
DIAGNOSTICS_FILE=$"Files/diag.txt"
DEPARTMENT_DATA_FILE=$"Files/department_data_file.xls"
EMPLOYEE_DATA_FILE="Files/employee_data_file.txt"
LEAVE_DATA_FILE="Files/leave_data_file.xlsx"
SALARY_DATA_FILE="Files/salary_data_file.xlsx"
OUTPUT_DATA_FILE="Files/output_data_file.txt"

cargo run --release -- \
--log-file ${LOG_FILE} \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--employee-data-file ${EMPLOYEE_DATA_FILE} \
--department-data-file ${DEPARTMENT_DATA_FILE} \
--leave-data-file ${LEAVE_DATA_FILE} \
--salary-data-file ${SALARY_DATA_FILE} \
--output-data-file ${OUTPUT_DATA_FILE} \
--log-level trace \
--diagnostics-flag false
