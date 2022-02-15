struct ScheduleGenerator {
    hours: Vec<i32>,
    step: usize,
    fill: bool,
    include: bool,
    delimiter: String,
}

impl Default for ScheduleGenerator {
    fn default() -> ScheduleGenerator {
        ScheduleGenerator {
            hours: vec![],
            step: 30,
            fill: true,
            include: true,
            delimiter: ":".to_string(),
        }
    }
}

impl Component for ScheduleGenerator {
    fn generate(&mut self) -> Vec<String> {
        let mut time_schedules = Vec::new();
        for hour in &self.hours {
            for minute in (0..60).step_by(self.step) {
                if self.fill{
                    time_schedules.push(format!("{}{}{}", format!("{:0>2}", hour), self.delimiter, format!("{:0>2}", minute)));
                } else {
                    time_schedules.push(format!("{}{}{}", hour, self.delimiter, minute));
                }
            }
        }

        if self.include {
            let mut hours_end = &0;
            match &self.hours.last() {
                Some(v) => hours_end = v,
                None => (),
            };
            
            if hours_end != &23 {
                if self.fill {
                    time_schedules.push(format!("{:?}{}00", hours_end + 1, self.delimiter));
                } else {
                    time_schedules.push(format!("{:?}{}0", hours_end + 1, self.delimiter));
                }
            }
        }
        return time_schedules
    }

    fn delete(&mut self) {}
    fn addition(&mut self) {}
    fn reorder(&mut self) {}
}
trait Component {
    fn generate(&mut self) -> Vec<String>;
    fn delete(&mut self);
    fn addition(&mut self);
    fn reorder(&mut self);
}

pub fn main() {
    let hours = (9..18).collect::<Vec<i32>>();
    let mut generator = ScheduleGenerator{hours, ..Default::default()};
    let result = &generator.generate();
    println!("{:?}", result);
}
