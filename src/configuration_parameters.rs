use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    employee_data_file_path: String,
    department_data_file_path: String,
    salary_data_file_path: String,
    leave_data_file_path: String,
    output_data_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "employee_data_file: {}", self.employee_data_file_path());
        info!(logger, "department_data_file: {}", self.department_data_file_path());
        info!(logger, "salary_data_file: {}", self.salary_data_file_path());
        info!(logger, "leave_data_file: {}", self.leave_data_file_path());
        info!(logger, "output_data_file: {}", self.output_data_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let employee_data_file_path = matches
            .value_of("employee_data_file_path")
            .expect("Error getting `employee_data_file_path`.")
            .to_string();
        let department_data_file_path = matches
            .value_of("department_data_file_path")
            .expect("Error getting `department_data_file_path`.")
            .to_string();
        let salary_data_file_path = matches
            .value_of("salary_data_file_path")
            .expect("Error getting `salary_data_file_path`.")
            .to_string();
        let leave_data_file_path = matches
            .value_of("leave_data_file_path")
            .expect("Error getting `leave_data_file_path`.")
            .to_string();
        let output_data_file_path = matches
            .value_of("output_data_file_path")
            .expect("Error getting `output_data_file_path`.")
            .to_string();
        ConfigurationParameters {
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            employee_data_file_path,
            department_data_file_path,
            salary_data_file_path,
            leave_data_file_path,
            output_data_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn employee_data_file_path(&self) -> &str {
        &self.employee_data_file_path
    }
    pub fn department_data_file_path(&self) -> &str {
        &self.department_data_file_path
    }
    pub fn salary_data_file_path(&self) -> &str {
        &self.salary_data_file_path
    }
    pub fn leave_data_file_path(&self) -> &str {
        &self.leave_data_file_path
    }
    pub fn output_data_file_path(&self) -> &str {
        &self.output_data_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program on how to read and write files")
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("employee_data_file_path")
                .long("employee-data-file")
                .value_name("Employee Data File Path")
                .help("Path to write employee data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("department_data_file_path")
                .long("department-data-file")
                .value_name("Department Data File Path")
                .help("Path to write department data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("salary_data_file_path")
                .long("salary-data-file")
                .value_name("Salary Data File Path")
                .help("Path to write salary data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("leave_data_file_path")
                .long("leave-data-file")
                .value_name("Leave Data File Path")
                .help("Path to write leave data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_data_file_path")
                .long("output-data-file")
                .value_name("Output data file path")
                .help("Path to write output data file.")
                .required(true)
        )
        .get_matches()
}