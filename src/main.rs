use std::io;


fn main() {
    
    println!("Welcome to habit tracker !");

    'tracker: loop {
        println!("1.Add Habit \n 2.View Habits \n 3.Complete Habit \n 4.View Stats \n 5.Reset stats \n 6.Exit");
        
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read a line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => println!("Entered adding habit function!"),
            2 => println!("Entered view habit function!"),
            3 => println!("Entered complete habit function!"),
            4 => println!("Entered view stats function!"),
            5 => println!("Entered reset stats function!"),
            6 => break 'tracker,
            0 | 7..=u32::MAX => println!("Invalid Choice !"),
    }

}
}
