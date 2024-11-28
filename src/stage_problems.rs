use crate::{
    init_stages::{hints, infos, introductions, problems},
    printer::center_text,
};

pub struct StageInfo {
    stage_complete: [(String, bool); 9],
    stage_hints: [&'static str; 9],
    stage_problems: [&'static str; 9],
    stage_introductions: [&'static str; 9],
    curr_stage: usize,
    stages: usize,
}

impl StageInfo {
    pub fn new() -> Self {
        StageInfo {
            stage_complete: infos::init_info(),
            stage_problems: problems::init_problems(),
            stage_hints: hints::init_hints(),
            stage_introductions: introductions::init_introductions(),
            curr_stage: 0,
            stages: 6,
        }
    }

    pub fn total_stages(&self) -> usize {
        self.stages
    }

    pub fn set_stage_completed(&mut self, num: usize) {
        self.curr_stage += 1;
        self.stage_complete[num].1 = true;
    }

    pub fn get_stage_complete_at(&self, num: usize) -> &(String, bool) {
        &self.stage_complete[num]
    }

    pub fn get_stage_hint(&self) -> &'static str {
        self.stage_hints[self.curr_stage]
    }

    pub fn get_introductions(&self, problem_num: usize) -> &'static str {
        if problem_num >= self.total_stages() {
            unreachable!("There should have been checks outside.");
        }

        self.stage_introductions[problem_num]
    }

    pub fn get_problem(&self, problem_num: usize) -> &'static str {
        if problem_num >= self.total_stages() {
            unreachable!("There should have been checks outside.");
        }

        self.stage_problems[problem_num]
    }

    pub fn print_progress_bar(&self) {
        let progress_num = self.curr_stage * 10;

        //let progress = "⬜".repeat(progress_num);
        let progress = "#".repeat(progress_num);
        let fill = "-".repeat(100 - progress_num);

        let mut progress_bar = String::new();

        progress_bar.push('[');
        progress_bar.push_str("\x1B[38;5;82m");
        progress_bar.push_str(&progress);
        progress_bar.push_str("\x1B[0m");
        progress_bar.push_str(&fill);
        progress_bar.push_str(&format!(" {}%", progress_num));
        progress_bar.push(']');
        progress_bar.push('\n');

        let progress_bar = center_text(&progress_bar, 19);

        println!("\n{}", progress_bar);
    }
}
