## Mini project for week 2: Playing rock-paper-scissors with computer!

The target of this project is to design a program for users to play rock-paper-scissors game with the computer. The goal is to get enough scores so as to win the game. In each round, you will need to input a choice for your next decision (either rock, paper or scissor). If tie occurs, both computer and user will not get score. Else, the winner will accumulate one point. The first one who gets the target number of scores will be the final winner. (You can decide the target number via the input variable tgt)

$` cargo run -- play --tgt u32`<br />

Sample input command line:

$`code$ cargo run -- play --tgt 3