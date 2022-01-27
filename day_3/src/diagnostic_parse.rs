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
    o2_rate: u32,
    co2_rate: u32,
    processed_lines: Vec<String>,
    proc_lines_count: u32
}

impl DiagnosticReport {
    pub fn new() -> DiagnosticReport {
        DiagnosticReport {
            gamma_rate: 0,
            epsilon_rate: 0,
            o2_rate: 0,
            co2_rate: 0,
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
        self.gamma_rate * self.epsilon_rate
    }

    pub fn get_oxigen_rate(&self) -> u32 {
        self.o2_rate
    }

    pub fn get_co2_rate(&self) -> u32 {
        self.co2_rate
    }

    pub fn get_life_support_rate(&self) -> u32 {
        self.o2_rate * self.co2_rate
    }

    pub fn process_line(&mut self, data: String) {
        self.processed_lines.push(data);
        self.proc_lines_count += 1;
        self.calculate_rates();
    }

    fn calculate_rates(&mut self) {
        self.calculate_power_consumption_rates();
        self.calculate_life_support_rates();
    }

    fn calculate_life_support_rates(&mut self) {
        self.o2_rate = self.calculate_o2_rate(&self.processed_lines, 0);
        self.co2_rate = self.calculate_co2_rate(&self.processed_lines, 0);
    }

    fn calculate_o2_rate(&self, data: &Vec<String>, mut idx: usize) -> u32 {
        if data.len() == 0 { return 0 }
        if data.len() == 1 { return self.binary_string_to_dec_number(&data[0]) }

        let filter_val = self.get_most_common_bit_in_idx(data, idx);
        let filtered_data = self.filter_by_idx_number(data, idx, filter_val);
        idx += 1;

        self.calculate_o2_rate(&filtered_data, idx)
    }

    fn calculate_co2_rate(&self, data: &Vec<String>, mut idx: usize) -> u32 {
        if data.len() == 0 { return 0 }
        if data.len() == 1 { return self.binary_string_to_dec_number(&data[0]) }

        let filter_val = self.get_most_common_bit_in_idx(data, idx);
        let filtered_data = self.filter_by_idx_number(data, idx, filter_val ^ 1);
        idx += 1;

        self.calculate_co2_rate(&filtered_data, idx)
    }

    #[deprecated]
    fn calculate_life_support_rate(&self, data: &Vec<String>, mut idx: usize, filter_val: u32) -> u32 {
        if data.len() == 0 { return 0 }
        if data.len() == 1 { return self.binary_string_to_dec_number(&data[0]) }

        let filtered_data = self.filter_by_idx_number(data, idx, filter_val);
        idx += 1;

        self.calculate_life_support_rate(&filtered_data, idx, filter_val)
    }

    fn filter_by_idx_number(&self, data: &Vec<String>, idx: usize, filter_val: u32) -> Vec<String> {
        let mut filtered_data: Vec<String> = Vec::new();
        for it in data.iter() {
            let bit = self.get_bit_at_position(it, idx);
            if bit == filter_val {
                filtered_data.push(it.to_string());
            }
        }

        filtered_data
    }

    fn get_most_common_bit_in_idx(&self, data: &Vec<String>, idx: usize) -> u32 {
        let mut zero_count = 0;
        for it in data.iter() {
            let bit = self.get_bit_at_position(it, idx);
            if bit == 0 { zero_count += 1 }
        }

        let zero_ratio = (zero_count as f64) / data.len() as f64;
        if zero_ratio > 0.5 { 0 }
        else { 1 }
    }

    fn calculate_power_consumption_rates(&mut self) {
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

    fn get_bit_at_position(&self, binary_num: &String, idx: usize) -> u32 {
        binary_num.chars()
            .nth(idx)
            .unwrap()
            .to_digit(2)
            .unwrap()
    }

    fn binary_string_to_dec_number(&self, binary_num: &String) -> u32 {
        let mut num = 0;
        for (idx, c) in binary_num.chars().rev().enumerate() {
            num += c.to_digit(2).unwrap() << idx;
        }

        num
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
    fn should_process_life_support_rates() {
        let diagnostic_report = DiagnosticReport::from_data(get_test_report_data());

        assert_eq!(diagnostic_report.get_oxigen_rate(), 23);
        assert_eq!(diagnostic_report.get_co2_rate(), 10);
        assert_eq!(diagnostic_report.get_life_support_rate(), 230);
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