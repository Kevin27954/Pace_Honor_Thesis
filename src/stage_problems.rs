pub struct StageInfo {
    stage_complete: [(String, bool); 4],
    stage_hints: [&'static str; 4],
    stage_problems: [&'static str; 4],
    curr_stage: usize,
    stages: usize,
}

impl StageInfo {
    pub fn new() -> Self {
        StageInfo {
            stage_complete: Self::init_info(),
            stage_problems: Self::init_problems(),
            stage_hints: Self::init_hints(),
            curr_stage: 0,
            stages: 4,
        }
    }

    pub fn total_stages(&self) -> usize {
        self.stages
    }

    pub fn get_stage_complete(&self) -> &[(String, bool)] {
        &self.stage_complete
    }

    pub fn set_stage_completed(&mut self, num: usize) {
        self.curr_stage += 1;
        self.stage_complete[num].1 = true;
        assert!(self.curr_stage <= self.stages);
    }

    pub fn get_stage_complete_at(&self, num: usize) -> &(String, bool) {
        &self.stage_complete[num]
    }

    pub fn get_stage_hint(&self) -> &'static str {
        self.stage_hints[self.curr_stage]
    }

    pub fn get_problem(&self, problem_num: usize) -> &'static str {
        if problem_num >= self.total_stages() {
            unreachable!("There should have been checks outside.");
        }

        self.stage_problems[problem_num]
    }

    fn init_info() -> [(String, bool); 4] {
        [
            (String::from("1_print.txt"), false),
            (String::from("2_number.txt"), false),
            (String::from("3_string.txt"), false),
            (String::from("4_math.txt"), false),
        ]
    }

    fn init_hints() -> [&'static str; 4] {
        [
            "\
Type  'print()'.  \n
Then  in  between  the parenthesis,
type  some  numbers  save. \n
Watch what happens in this place.
",
            "\
Type basic digits together like 0 to 9\n
There are also decimals, also known as
floating values. \n
Both are considered as numbers in
Bitelang.\n
Ex.
1234
1234.5678
-1234
-1234.5678
",
            "\
To use words in Bitelang we use \"\".\n
Any characters in between the \"\"
are accepted. \n
Use print(), that you learned in stage
1 to display the String.\n
Ex.
\"This is a sample String.\"
\"This too: 🐶.\"
\"Numbers too: 12345\"
",
            "\
Math operations are important in 
programming. Try doing math with 2
numbers with:
        +
        -
        *
        /
Remember to use print() to display
the result.\n
Ex.
print(34.33 - 59.199)
",
        ]
    }

    fn init_problems() -> [&'static str; 4] {
        [
            "\
// STILL LEARNING

// Play around with the print function. Get use to it.
// Try with numbers or multiple numbers:
// print(123)
// print(123, 456, 789)
",
            "\
// STILL LEARNING

Numbers are important in all aspects. 
",
            "\
// STILL LEARNING
Strings oonga boonga
",
            "\
// STILL LEARNING
Math operations
",
        ]
    }
}
