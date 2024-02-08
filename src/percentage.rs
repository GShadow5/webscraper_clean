use function_name::named;
use log::{debug, info};
pub struct Percentage {
    /*
    This struct is used to track the percentage of a task that has been completed.
    It is used to print progress to the console.
    */
    threshold: f32,
    step_size: f32,
    total: usize,
    count: usize,
    percent: f32,
}

impl Percentage {
    #[named]
    pub fn new(total: usize) -> Percentage {
        let mut step = 0.1;
        let mut threshold = 0.1;
        if total == 0 {
            debug!(
                "fn {} - Percentage::new() called with total = 0",
                function_name!()
            );
        }
        // If there are less than 10 items, set the step size to whatever percentage of the total each item represents
        if total < 10 {
            step = 1.0 / total as f32;
        }
        // If there are greater than 100 items, set the step size and threshold to 5%
        if total > 100 {
            step = 0.05;
            threshold = 0.05;
        }
        // If there are greater than 1000 items, set the step size and threshold to 1%
        if total > 1000 {
            step = 0.01;
            threshold = 0.01;
        }
        Percentage {
            threshold,
            step_size: step,
            total,
            count: 0,
            percent: 0.0,
        }
    }
    pub fn get_percent(&self) -> f32 {
        self.percent
    }
    pub fn get_total(&self) -> usize {
        self.total
    }
    pub fn update(&mut self, fn_name: &str) {
        if self.total == 0 {
            info!(target: "w10s_webscraper", "fn {} - Percentage done: zero items", fn_name);
        }
        // Update the progress
        self.count += 1;
        // Calculate the percentage
        let percent: f32 = self.count as f32 / self.total as f32;
        // If the percentage is greater than the threshold, print the percentage
        if percent >= self.threshold {
            info!(target: "w10s_webscraper", "fn {} - {:.0}%", fn_name, percent * 100.0);
            // Update the threshold
            self.threshold += self.step_size;
        }
    }
}
