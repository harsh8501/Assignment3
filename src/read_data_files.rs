extern crate calamine;

use configuration_parameters::{get_configuration_parameters,ConfigurationParameters};
use structs::{initial_employee_data,leave_report,Salary_report,final_report};
use self::calamine::{open_workbook, Error, Xlsx, Xls, Reader, RangeDeserializerBuilder,DataType};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

pub fn read_employee_file(app_name: &str) -> HashMap<String,initial_employee_data> {
    let config_params= get_configuration_parameters(app_name);
    let path = config_params.employee_data_file_path();
    let mut mapofemp:HashMap<String,initial_employee_data>=HashMap::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index==0 {
            continue;
        }
        let line = line.unwrap();
        let tokens:Vec<&str>= line.split("|").collect();
        let EmpId = tokens[0].to_string();
        let EmpName = tokens[1].to_string();
        let DeptId = tokens[2].to_string();
        let MobileNo = tokens[3].to_string();
        let Email = tokens[4].to_string();
        let employee = initial_employee_data{EmpId, EmpName, DeptId, MobileNo,Email};
        mapofemp.insert(tokens[0].to_string(),employee );
    } 
    return mapofemp;
 }

pub fn read_department_file(app_name: &str)->HashMap<String,String> {
    let config_params = get_configuration_parameters(app_name);
    let path = config_params.department_data_file_path();
    let mut workbook: Xls<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofdept=HashMap::new();
    while let Some(result) = iter.next() {
        let (DeptID, Dept_title, Dept_Strength): (String, String, String) = result.unwrap();
        mapofdept.insert(DeptID, Dept_title);
    }
    return mapofdept;
 }



 pub fn read_salary_file(app_name: &str)->HashMap<String,Salary_report> {
    let config_params = get_configuration_parameters(app_name);
    let path = config_params.salary_data_file_path();
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofsalary:HashMap<String, Salary_report >=HashMap::new();
    while let Some(result) = iter.next() {
        let (EmpId, SalaryID, Salarydate, salary): (String, String, String,String) = result.unwrap();
        let SalaryId = SalaryID;
        let SalaryDate = Salarydate;
        let Salary = salary;
        let SalaryStatusReport = Salary_report { SalaryId, SalaryDate, Salary};
        mapofsalary.insert(EmpId, SalaryStatusReport);
    }
    return mapofsalary;
 }

 pub fn read_leave_file(app_name: &str)->HashMap<String,leave_report> {
    let config_params = get_configuration_parameters(app_name);
    let path = config_params.leave_data_file_path();
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofleave:HashMap<String, leave_report >=HashMap::new();
    while let Some(result) = iter.next() {
        let (EmpId, leaveID, leavefrom, leaveto, leavetype): (String, String, String,String,String) = result.unwrap();
        let LeaveId = leaveID;
        let LeaveFrom = leavefrom;
        let LeaveTo = leaveto;
        let LeaveType = leavetype;
        let LeaveStatusReport = leave_report { LeaveId, LeaveFrom, LeaveTo, LeaveType};
        mapofleave.insert(EmpId, LeaveStatusReport);
    }
    return mapofleave;
 }

 pub fn get_empid_dept_id_pair(app_name: &str) ->HashMap<String, String> {
    let mut tempmap:HashMap<String, initial_employee_data >=HashMap::new();
    tempmap = read_employee_file(app_name);
    let mut empid_deptid_pair:HashMap<String, String>=HashMap::new();
    for (key, val) in tempmap.iter(){
        empid_deptid_pair.insert(key.to_string(), val.DeptId.to_string());
    }
    return empid_deptid_pair;
 } 