use std::io;

struct Habit {
    name: String,
    completed: u32,
}

impl Habit {

    fn complete(&mut self) {
        self.completed += 1;
    }

    fn display(&self) {
        println!("{} completed {} times.",self.name,self.completed);
    }

    fn display_habit(&self , index : u32) {
        println!("{index}. {}",self.name);
    }
}

fn add_habit(habits:&mut Vec<Habit>) {
    let mut input = Habit {
        name: String::new(),
        completed: 0
    };
    
    println!("Enter the habit to track ");

    io::stdin()
        .read_line(&mut input.name)
        .expect("Failed to read a line");
    
    input.name = input.name.trim().to_string();
    habits.push(input);
    println!("Added the habit ");

}

fn view_habit(habits: &Vec<Habit>) {
    println!("These are the habits are being recorded ");

    if habits.is_empty() {
        println!("Add habits to view !");
    
    } else {
    for (index,habit) in habits.iter().enumerate() {
        habit.display_habit(index.try_into().unwrap());
    }
    }
}

fn complete_habit(habits: &mut Vec<Habit>) {
    println!("Enter the id of the habit to mark it complete \n");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read a line");

    let index:u32 = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return ,
    };

    for (ind,habit) in habits.iter_mut().enumerate() {
        if index == ind.try_into().unwrap() {
            habit.complete();
            habit.display();
        } else {
            continue ; 
        }
    }
}

fn main() {
    
    println!("Welcome to habit tracker !");
    
    let mut habits: Vec<Habit> = Vec::new();

    'tracker: loop {
        println!(" 1.Add Habit \n 2.View Habits \n 3.Complete Habit \n 4.View Stats \n 5.Reset stats \n 6.Exit");
        
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read a line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        match choice {
            1 => add_habit(&mut habits),
            2 => view_habit(&habits),
            3 => complete_habit(&mut habits),
            4 => println!("Entered view stats function!"),
            5 => println!("Entered reset stats function!"),
            6 => break 'tracker,
            0 | 7..=u32::MAX => println!("Invalid Choice !"),
    }

}
}
