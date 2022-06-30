extern crate chrono;
extern crate calamine;

use configuration_parameters::{get_configuration_parameters,ConfigurationParameters};
use read_data_files::{read_department_file,read_employee_file,read_leave_file,read_salary_file,get_empid_dept_id_pair};
use self::calamine::{open_workbook, Error, Xlsx, Xls, Reader, RangeDeserializerBuilder,DataType};
use structs::{initial_employee_data,Salary_report,leave_report,final_report};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use self::chrono::prelude::*;
use self::chrono::{DateTime, FixedOffset, TimeZone, NaiveDate};

pub fn get_final_report(app_name:&str) -> HashMap<String,final_report> {
    let localcurrentmonth: DateTime<Local> = Local::now();
    let currentmonnth = localcurrentmonth.format("%b %Y",).to_string();
    let mut mapoffinalreport:HashMap<String, final_report >=HashMap::new();
    let mut mapofemp:HashMap<String,initial_employee_data>=HashMap::new();
    let mut mapofdept:HashMap<String,String>=HashMap::new();
    let mut mapofsalary:HashMap<String, Salary_report >=HashMap::new();
    let mut mapofleave:HashMap<String, leave_report >=HashMap::new();
    let mut empid_deptid_pair:HashMap<String, String>=HashMap::new();
    mapofemp = read_employee_file(app_name);
    mapofdept = read_department_file(app_name);
    mapofsalary = read_salary_file(app_name);
    mapofleave = read_leave_file(app_name);
    empid_deptid_pair = get_empid_dept_id_pair(app_name);
    for(key,value) in empid_deptid_pair.iter(){
        let employeeid = key;
        let EmployeedeptId = value;
        let DeptTitle = mapofdept.get(EmployeedeptId).unwrap().to_string();
        let empdetail = mapofemp.get(employeeid).unwrap();
        let empsalarydetail = mapofsalary.get(employeeid).unwrap();
        let empleavedetail = mapofleave.get(employeeid).unwrap();
        let mut SalaryStatus = "".to_string();
        if empsalarydetail.Salary!=""{
            let lastsalarymonth = empsalarydetail.SalaryDate.to_string();
            let check = currentmonnth.eq(&lastsalarymonth);
            if check==true {
                SalaryStatus = "Credited".to_string();
            }
            else{
                SalaryStatus = "Not Credited".to_string();
            }
        }
        else {
            SalaryStatus = "Not Credited".to_string();
        }
        let EmpId = empdetail.EmpId.to_string();
        let EmpName = empdetail.EmpName.to_owned();
        let MobileNo = empdetail.MobileNo.to_owned();
        let Email = empdetail.Email.to_owned();
        let employeeleavefrom = empleavedetail.LeaveFrom.to_string();
        let employeeleaveto = empleavedetail.LeaveTo.to_string();
        let mut OnLeave = "".to_string();
        if employeeleavefrom !="" {
        let naiveleavefrom = NaiveDate::parse_from_str(&employeeleavefrom, "%d-%m-%Y").unwrap();
        let naiveleaveto = NaiveDate::parse_from_str(&employeeleaveto, "%d-%m-%Y").unwrap();
        OnLeave = naiveleaveto.signed_duration_since(naiveleavefrom).num_days().to_string();
        }
        else {
            OnLeave = "0".to_string();
        }
        let employee = final_report{EmpId,EmpName,DeptTitle,MobileNo,Email,SalaryStatus,OnLeave};
        mapoffinalreport.insert(key.to_string(), employee);
    }
    return mapoffinalreport;
 }
 pub fn output_data(app_name:&str){
    let mut mapoffinalreport:HashMap<String, final_report >=HashMap::new();
    mapoffinalreport = get_final_report(app_name);
    let config_params= get_configuration_parameters(app_name);
    let path = config_params.output_data_file_path();
    let mut Summary = std::fs::File::create(path).expect("create failed");
    for (key,value) in mapoffinalreport.iter(){
        let ID = value.EmpId.to_owned();
        let name = value.EmpName.to_owned();
        let depttitle = value.DeptTitle.to_owned();
        let mobile = value.MobileNo.to_owned();
        let email = value.Email.to_owned();
        let salarystatus = value.SalaryStatus.to_owned();
        let onleave = value.OnLeave.to_owned();
        let delimiter = "~#~".to_string();
        let v = vec![ID,name,depttitle,mobile,email,salarystatus,onleave];
        let s = v.connect("~#~");
        Summary.write_all(s.as_bytes());
        Summary.write_all("\n".as_bytes());
    }
 }