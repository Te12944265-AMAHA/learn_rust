use std::collections::HashMap;

struct Company {
    roster: HashMap<String, Vec<String>>,
}

pub fn simulate_company() {
    let mut company = Company {
        roster: HashMap::new(),
    };
    company.add_employee_text("Add Sally to Engineering");
    company.add_employee_text("Add Amir to Sales");
    company.add_employee_text("Add God to Engineering");
    company.add_employee_text("Add Bobby to Engineering");
    company.add_employee_text("Add Badboy to Management");

    println!("Printing engineering department roster..");
    company.print_roster("Engineering");

    println!("\nPrinting company roster..");
    company.print_full_roster();
}

impl Company {
    pub fn add_employee_text(&mut self, text: &str) {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() != 4 {
            panic!("Wrong text format");
        }
        if words[0] != "Add" || words[2] != "to" {
            panic!("Wrong text format");
        }
        let name = &words[1];
        let dep = &words[3];
        self.add_employee(name, dep);
    }

    pub fn print_full_roster(&self) {
        for (dep, dep_roster) in &(self.roster) {
            println!("List of people in {} department: {:#?}", dep, dep_roster);
        }
    }

    pub fn print_roster(&mut self, dep: &str) {
        if self.roster.contains_key(dep) {
            let roster_copy = self.roster.get_mut(dep).unwrap();

            roster_copy.reverse();
            println!("List of people in {} department: {:#?}", dep, roster_copy);
        } else {
            println!("No such department: {}", dep);
        }
    }

    fn add_employee(&mut self, name: &str, dep: &str) {
        // roster: dep name string -> vector of names
        self.roster.entry(String::from(dep)).or_insert(Vec::new());
        self.roster.get_mut(dep).unwrap().push(String::from(name));
    }
}
