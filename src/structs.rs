#[derive(Debug)]
pub struct initial_employee_data {
    pub EmpId: String,
    pub EmpName: String,
    pub DeptId: String,
    pub MobileNo: String,
    pub Email : String
}

#[derive(Debug)]
pub struct leave_report{
    pub LeaveId: String,
    pub LeaveFrom: String,
    pub LeaveTo: String,
    pub LeaveType: String
}

#[derive(Debug)]
pub struct Salary_report{
    pub SalaryId: String,
    pub SalaryDate: String,
    pub Salary: String,
}

#[derive(Debug)]
pub struct final_report {
    pub EmpId: String,
    pub EmpName: String,
    pub DeptTitle: String,
    pub MobileNo: String,
    pub Email : String,
    pub SalaryStatus: String,
    pub OnLeave: String
} 