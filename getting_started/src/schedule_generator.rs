pub trait Components {
    fn generate(&mut self);
    fn delete(&mut self, del_schedules: Vec<String>);
    fn addition(&mut self, add_schedules: Vec<String>);
}

pub struct ScheduleGenerator {
    pub hours: Vec<i32>,
    pub time_schedules: Vec<String>,
    pub step: usize,
    pub fill: bool,
    pub include: bool,
    pub delimiter: String,
}

impl Default for ScheduleGenerator {
    fn default() -> ScheduleGenerator {
        ScheduleGenerator {
            hours: Vec::new(),
            time_schedules: Vec::new(), 
            step: 30,
            fill: true,
            include: true,
            delimiter: ":".to_string(),
        }
    }
}

impl Components for ScheduleGenerator {
    fn generate(&mut self) {
        let mut time_schedules = Vec::new();
        for hour in &self.hours {
            for minute in (0..60).step_by(self.step) {
                if self.fill {
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
        self.time_schedules = time_schedules;
    }

    fn delete(&mut self, del_schedules: Vec<String>) {
        let mut tmp = Vec::new();

        for time_schedule in &self.time_schedules {
            tmp.push(format!("{}", time_schedule));
        }

        for del_schedule in del_schedules {
            let index = tmp.iter().position(|x| *x == del_schedule).unwrap();
            tmp.remove(index);
        }
        self.time_schedules = Vec::new();
        self.time_schedules = tmp;
    }
    fn addition(&mut self, add_schdules: Vec<String>) {
        for add_schdule in add_schdules {
            self.time_schedules.push(add_schdule);
        }
    }
}
