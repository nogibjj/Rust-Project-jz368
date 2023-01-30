use rand::Rng;
use std::io;
pub fn int2name(choice: u32) ->  &'static str {
    if choice==1{
        return "rock";
    }
    if choice==2{
        return "paper";
    }
    if choice==3{
        return "scissors";
    }
    return "None";
}

pub fn game(tgt: u32) {
    let mut round = 0;
    let mut computer_wins = 0;
    let mut user_wins = 0;

    println!("Target score: {tgt}");

    loop {
        round = round + 1;
        println!("Round {round}. ");
        println!("Input your choice for this round: (1: rock; 2: Paper; 3: Scissors)");

        let mut userchoice = String::new();
        io::stdin()
            .read_line(&mut userchoice)
            .expect("Failed to understand the user choice");
        let userchoice: u32 = userchoice.trim().parse().expect("Please type a valid choice!");
        let computerchoice = rand::thread_rng().gen_range(1..4);

        let name1 = int2name(computerchoice);
        println!("Computer Choice: {name1}");
        let name2 = int2name(userchoice);
        println!("User Choice: {name2}");

        if computerchoice==userchoice{
            println!("Tie.");
        }
        if computerchoice==2 && userchoice==1{
            computer_wins = computer_wins+1;
        }
        if computerchoice==1 && userchoice==2{
            user_wins = user_wins+1;
        }
        if computerchoice==3 && userchoice==2{
            computer_wins = computer_wins+1;
        }
        if computerchoice==2 && userchoice==3{
            user_wins = user_wins+1;
        }
        if computerchoice==1 && userchoice==3{
            computer_wins = computer_wins+1;
        }
        if computerchoice==3 && userchoice==1{
            user_wins = user_wins+1;
        }
        println!("Current scores (computer vs user): {computer_wins}:{user_wins}");
        println!("");
        if computer_wins == tgt {
            println!("Computer wins.");
            break;
        }
        if user_wins == tgt {
            println!("You wins");
            break;
        }
    }
}
