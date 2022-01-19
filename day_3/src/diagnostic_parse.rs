#[deprecated()]
pub fn parse_report_line(data: &str) -> u32 {
    let mut num: u32 = 0;
    for (idx, binary_item) in data.chars().rev().enumerate() {
        let binary_num = binary_item.to_digit(2).unwrap();
        num += binary_num << idx;
    }

    return num;
}

pub struct DiagnosticReport {
    gamma_rate: u32,
    epsilon_rate: u32,
    processed_lines: Vec<String>,
    proc_lines_count: u32
}

impl DiagnosticReport {
    pub fn new() -> DiagnosticReport {
        DiagnosticReport {
            gamma_rate: 0,
            epsilon_rate: 0,
            processed_lines: Vec::new(),
            proc_lines_count: 0
        }
    }

    pub fn from_data(data: Vec<String>) -> DiagnosticReport {
        let mut diagnostic_repo = DiagnosticReport::new();
        for line in data {
            diagnostic_repo.process_line(line);
        }

        diagnostic_repo
    }

    pub fn get_gamma_rate(&self) -> u32 {
        self.gamma_rate
    }

    pub fn get_epsilon_rate(&self) -> u32 {
        self.epsilon_rate
    }

    pub fn get_power_consumption(&self) -> u32 {
        self.gamma_rate as u32 * self.epsilon_rate as u32
    }

    pub fn process_line(&mut self, data: String) {
        self.processed_lines.push(data);
        self.proc_lines_count += 1;
        self.calculate_rates();
    }

    fn calculate_rates(&mut self) {
        let zeroes_count = self.calculate_data_zeroes_count();
        let mut gamma_rate: u32 = 0;
        let mut epsilon_rate: u32 = 0;
        for (idx, count) in zeroes_count.iter().enumerate() {
            let zero_ratio = (*count as f64) / self.proc_lines_count as f64;
            let most_common = if zero_ratio > 0.5 { 0 } else { 1 };
            let least_common = most_common ^ 1;

            gamma_rate += most_common << idx;
            epsilon_rate += least_common << idx;
        }

        self.epsilon_rate = epsilon_rate;
        self.gamma_rate = gamma_rate;
    }

    fn calculate_data_zeroes_count(&mut self) -> Vec<u32> {
        let mut zeroes_count: Vec<u32> = Vec::new();
        for line in self.processed_lines.iter() {
            self.add_zeroes_to_count(&line, &mut zeroes_count);
        }

        zeroes_count
    }

    fn add_zeroes_to_count(&self, data: &str, zeroes_count: &mut Vec<u32>) {
        self.adjust_zeroes_count_size(zeroes_count, &data.len());
        for (idx, binary_item) in data.chars().rev().enumerate() {
            let bit = binary_item.to_digit(2).unwrap();
            if bit == 0 {
                zeroes_count[idx] += 1;
            }
        }
    }

    fn adjust_zeroes_count_size(&self, zeroes_count: &mut Vec<u32>, size: &usize) {
        if zeroes_count.len() < *size {
            zeroes_count.extend(vec![0; *size - zeroes_count.len()]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DiagnosticReport;

    #[test]
    fn should_process_diagnostic_gamma_and_epsilon() {
        let diagnostic_report = DiagnosticReport::from_data(get_test_report_data());

        assert_eq!(diagnostic_report.get_gamma_rate(), 22);
        assert_eq!(diagnostic_report.get_epsilon_rate(), 9);
        assert_eq!(diagnostic_report.get_power_consumption(), 198);
    }

    #[test]
    fn should_process_random_size_report_lines() {
        let report_data = vec![
            "01101000".to_string(),
            "00110011".to_string(),
            "10011001".to_string()
        ];
        let diagnostic_report = DiagnosticReport::from_data(report_data);

        assert_eq!(diagnostic_report.get_gamma_rate(), 57); // 00111001
        assert_eq!(diagnostic_report.get_epsilon_rate(), 198); // 11000110
        assert_eq!(diagnostic_report.get_power_consumption(), 11286);
    }

    fn get_test_report_data() -> Vec<String> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string()
        ]
    }
}