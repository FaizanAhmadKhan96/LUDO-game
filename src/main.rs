use std::io;
use rand::Rng;
use std::collections::HashMap;

const DICE:char='\u{1F3B2}';
const SNAKE:char='\u{1F40D}';
const WARN:char='\u{26A0}';
const CONGRATS:char='\u{1F387}';

fn main()
{
    let mut turn=1;

println!("Welcome to Snake {} and Ladder game!",SNAKE);

loop
{
println!("Enter number of Players:");
let mut player_no = String::new();
io::stdin().read_line(&mut player_no);
let player_no: u8= match player_no.trim().parse() {
Ok(num) => num,
Err(_) => {println!("Only Numbers allowed,{} Try again!",WARN); 
                                  continue;},
};

    let mut score_player_1 = 0;
    let mut score_player_2 = 0;
    let mut score_player_3 = 0;
    let mut score_player_4 = 0;

    if player_no == 2
{
    println!("Enter name of Player 1:");
    let mut player_1 = String::new();
    io::stdin().read_line(&mut player_1).expect("Error!");

    println!("Enter name of Player 2:");
    let mut player_2 = String::new();
    io::stdin().read_line(&mut player_2).expect("Error!");

    println!("Player:{} Player:{}",player_1,player_2);
    println!("{}:0 {}:0",player_1,player_2);

    loop
    {
  
      let mut dice_1=rand::thread_rng().gen_range(1, 7);
      let mut dice_2=rand::thread_rng().gen_range(1, 7);
  
      score_player_1 = score_player_1 + dice_1;
      score_player_2 = score_player_2 + dice_2;
  
      println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_1,dice_1,score_player_1);
      println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_2,dice_2,score_player_2);
  
      if score_player_1 == score_player_2
      {
          score_player_1=0;
          println!("Alas,{} was kicked by {}",player_1,player_2);
      }
  
      else if score_player_2 == score_player_1
      {
          score_player_2=0;
          println!("Alas,{} was kicked by {}",player_2,player_1);
      }
  
      if dice_1==6
      {
        dice_1 = dice_1 + rand::thread_rng().gen_range(1, 7);
        score_player_1 = score_player_1 + dice_1;

          if dice_1==6
          {
              dice_1=0;
              score_player_1 = score_player_1 + dice_1;
              println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_1);
          }
      }
  
       else if dice_2==6
       {
        dice_2 = dice_2 + rand::thread_rng().gen_range(1, 7);
        score_player_2 = score_player_2 + dice_2;

         if dice_2==6
         {
             dice_2=0;
             score_player_2 = score_player_2 + dice_2;
             println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_2);
         }
      }
  
          if score_player_1 == 100
          {
            println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_1,turn);
            println!("{}:{} {}:{}",player_1,score_player_1,player_2,score_player_2);
            break;
          }
          
          else  if score_player_2 == 100
          {
            println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_2,turn);
            println!("{}:{} {}:{}",player_1,score_player_1,player_2,score_player_2);
            break;
          }
  
          if score_player_1 < 100
          {
          score_player_1 = score_player_1;
          }
  
          else if score_player_2 < 100
          {
          score_player_2 = score_player_2;
          }
  
          if score_player_1 > 100
          {
          score_player_1 = score_player_1 - dice_1;
          }
  
          else if score_player_2 > 100
          {
          score_player_2 = score_player_2 - dice_2;
          }
  
      turn += 1;
  
      println!("Press Enter!");
      let mut enter = String::new();
      io::stdin().read_line(&mut enter);
    }
}

else if player_no == 3
{
    println!("Enter name of Player 1:");
    let mut player_1 = String::new();
    io::stdin().read_line(&mut player_1).expect("Error!");

    println!("Enter name of Player 2:");
    let mut player_2 = String::new();
    io::stdin().read_line(&mut player_2).expect("Error!");

    println!("Enter name of Player 3:");
    let mut player_3 = String::new();
    io::stdin().read_line(&mut player_3).expect("Error!");

    println!("Player:{} Player:{} Player:{}",player_1,player_2,player_3);
    println!("{}:0 {}:0 {}:0",player_1,player_2,player_3);

    loop
  {

    let mut dice_1=rand::thread_rng().gen_range(1, 7);
    let mut dice_2=rand::thread_rng().gen_range(1, 7);
    let mut dice_3=rand::thread_rng().gen_range(1, 7);

    score_player_1 = score_player_1 + dice_1;
    score_player_2 = score_player_2 + dice_2;
    score_player_3 = score_player_3 + dice_3;

    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_1,dice_1,score_player_1);
    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_2,dice_2,score_player_2);
    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_3,dice_3,score_player_3);

    if score_player_1 == score_player_2
    {
        score_player_1=0;
        println!("Alas,{} was kicked by {}",player_1,player_2);
    }

    else if score_player_1 == score_player_3
    {
        score_player_1=0;
        println!("Alas,{} was kicked by {}",player_1,player_3);
    }

    else if score_player_2 == score_player_1
    {
        score_player_2=0;
        println!("Alas,{} was kicked by {}",player_2,player_1);
    }

    else if score_player_2 == score_player_3
    {
        score_player_2=0;
        println!("Alas,{} was kicked by {}",player_2,player_3);
    }

    else if score_player_3 == score_player_1
    {
        score_player_3=0;
        println!("Alas,{} was kicked by {}",player_3,player_1);
    }

    else if score_player_3 == score_player_2
    {
        score_player_3=0;
        println!("Alas,{} was kicked by {}",player_3,player_2);
    }

    if dice_1==6
    {
        dice_1 = dice_1 + rand::thread_rng().gen_range(1, 7);
        score_player_1 = score_player_1 + dice_1;

        if dice_1==6
        {
            dice_1=0;
            score_player_1 = score_player_1 + dice_1;
            println!("Alas, Thrice Sixer! Dice roll{} of {} is 0",DICE,player_1);
        }
    }

     else if dice_2==6
     {
        dice_2 = dice_2 + rand::thread_rng().gen_range(1, 7);
        score_player_2 = score_player_2 + dice_2;

       if dice_2==6
       {
           dice_2=0;
           score_player_2 = score_player_2 + dice_2;
           println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_2);
       }
    }

    else if dice_3==6
    {
      dice_3 = dice_3 + rand::thread_rng().gen_range(1, 7);
      score_player_3 = score_player_3 + dice_3;

      if dice_3==6
      {
          dice_3=0;
          score_player_3 = score_player_3 + dice_3;
          println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_3);
      }
   }

        if score_player_1 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_1,turn);
          println!("{}:{} {}:{} {}:{}",player_1,score_player_1,player_2,score_player_2,player_3,score_player_3);
          break;
        }
        
        else  if score_player_2 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_2,turn);
          println!("{}:{} {}:{} {}:{}",player_1,score_player_1,player_2,score_player_2,player_3,score_player_3);
          break;
        }

        else  if score_player_3 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_3,turn);
          println!("{}:{} {}:{} {}:{}",player_1,score_player_1,player_2,score_player_2,player_3,score_player_3);
          break;
        }

        if score_player_1 < 100
        {
        score_player_1 = score_player_1;
        }

        else if score_player_2 < 100
        {
        score_player_2 = score_player_2;
        }

        else if score_player_3 < 100
        {
        score_player_3 = score_player_3;
        }

        if score_player_1 > 100
        {
        score_player_1 = score_player_1 - dice_1;
        }

        else if score_player_2 > 100
        {
        score_player_2 = score_player_2 - dice_2;
        }

        else if score_player_3 > 100
        {
        score_player_3 = score_player_3 - dice_3;
        }

    turn += 1;

    println!("Press Enter!");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter);
  }
}

 else if player_no == 4
 {
    println!("Enter name of Player 1:");
    let mut player_1 = String::new();
    io::stdin().read_line(&mut player_1).expect("Error!");

    println!("Enter name of Player 2:");
    let mut player_2 = String::new();
    io::stdin().read_line(&mut player_2).expect("Error!");

    println!("Enter name of Player 3:");
    let mut player_3 = String::new();
    io::stdin().read_line(&mut player_3).expect("Error!");

    println!("Enter name of Player 4:");
    let mut player_4 = String::new();
    io::stdin().read_line(&mut player_4).expect("Error!");

    let mut names: Vec<_> = Vec::new();
    names.push(&player_1);
    names.push(&player_2);
    names.push(&player_3);
    names.push(&player_4);

    for element in names
    {
        println!("Player:{}",element);
    }

    println!("{}:0 {}:0 {}:0 {}:0",player_1,player_2,player_3,player_4);

  loop
  {

    let mut dice_1=rand::thread_rng().gen_range(1, 7);
    let mut dice_2=rand::thread_rng().gen_range(1, 7);
    let mut dice_3=rand::thread_rng().gen_range(1, 7);
    let mut dice_4=rand::thread_rng().gen_range(1, 7);

    score_player_1 = score_player_1 + dice_1;
    score_player_2 = score_player_2 + dice_2;
    score_player_3 = score_player_3 + dice_3;
    score_player_4 = score_player_4 + dice_4;

    let mut score_table = HashMap::new();
    score_table.insert(&player_1,score_player_1);
    score_table.insert(&player_2,score_player_2);
    score_table.insert(&player_3,score_player_3);
    score_table.insert(&player_4,score_player_4);

    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_1,dice_1,score_player_1);
    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_2,dice_2,score_player_2);
    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_3,dice_3,score_player_3);
    println!("Turn {} Dice Roll {} of {} is {} and total Score is{}",turn,DICE,player_4,dice_4,score_player_4);

    if score_player_1 == score_player_2
    {
        score_player_1=0;
        println!("Alas,{} was kicked by {}",player_1,player_2);
    }

    else if score_player_1 == score_player_3
    {
        score_player_1=0;
        println!("Alas,{} was kicked by {}",player_1,player_3);
    }

    else if score_player_1 == score_player_4
    {
        score_player_1=0;
        println!("Alas,{} was kicked by {}",player_1,player_4);
    }

    else if score_player_2 == score_player_1
    {
        score_player_2=0;
        println!("Alas,{} was kicked by {}",player_2,player_1);
    }

    else if score_player_2 == score_player_3
    {
        score_player_2=0;
        println!("Alas,{} was kicked by {}",player_2,player_3);
    }

    else if score_player_2 == score_player_4
    {
        score_player_2=0;
        println!("Alas,{} was kicked by {}",player_2,player_4);
    }

    else if score_player_3 == score_player_1
    {
        score_player_3=0;
        println!("Alas,{} was kicked by {}",player_3,player_1);
    }

    else if score_player_3 == score_player_2
    {
        score_player_3=0;
        println!("Alas,{} was kicked by {}",player_3,player_2);
    }

    else if score_player_3 == score_player_4
    {
        score_player_3=0;
        println!("Alas,{} was kicked by {}",player_3,player_4);
    }

    else if score_player_4 == score_player_1
    {
        score_player_4=0;
        println!("Alas,{} was kicked by {}",player_4,player_1);
    }

    else if score_player_4 == score_player_2
    {
        score_player_4=0;
        println!("Alas,{} was kicked by {}",player_4,player_2);
    }

    else if score_player_4 == score_player_3
    {
        score_player_4=0;
        println!("Alas,{} was kicked by {}",player_4,player_3);
    }

    if dice_1==6
    {
        if dice_1==6
        {
            dice_1=0;
            score_player_1 = score_player_1 + dice_1;
            println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_1);
        }
        dice_1 = dice_1 + rand::thread_rng().gen_range(1, 7);
        score_player_1 = score_player_1 + dice_1;
    }

     else if dice_2==6
     {
        dice_2 = dice_2 + rand::thread_rng().gen_range(1, 7);
        score_player_2 = score_player_2 + dice_2;
       if dice_2==6
       {
           dice_2=0;
           score_player_2 = score_player_2 + dice_2;
           println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_2);
       }
    }

    else if dice_3==6
    {
      dice_3 = dice_3 + rand::thread_rng().gen_range(1, 7);
      score_player_3 = score_player_3 + dice_3;
      if dice_3==6
      {
          dice_3=0;
          score_player_3 = score_player_3 + dice_3;
          println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_3);
      }
   }

   else if dice_4==6
    {
     dice_4 = dice_4 + rand::thread_rng().gen_range(1, 7);
     score_player_4 = score_player_4 + dice_4;
     if dice_4==6
     {
         dice_4=0;
         score_player_4 = score_player_4 + dice_4;
         println!("Alas, Thrice Sixer! Dice roll {} of {} is 0",DICE,player_4);
     }
     }

        if score_player_1 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_1,turn);
          for (key,value) in &score_table
          {
              println!("{}:{}",key,value);
          }
          break;
        }
        
        else  if score_player_2 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_2,turn);
          for (key,value) in &score_table
          {
              println!("{}:{}",key,value);
          }
          break;
        }

        else  if score_player_3 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_3,turn);
          for (key,value) in &score_table
          {
              println!("{}:{}",key,value);
          }
          break;
        }

        else  if score_player_4 == 100
        {
          println!("{} Congratulations! Player {} has worn on turn {}",CONGRATS,player_4,turn);
          for (key,value) in &score_table
          {
              println!("{}:{}",key,value);
          }
          break;
        }

        if score_player_1 < 100
        {
        score_player_1 = score_player_1;
        }

        else if score_player_2 < 100
        {
        score_player_2 = score_player_2;
        }

        else if score_player_3 < 100
        {
        score_player_3 = score_player_3;
        }

        else if score_player_4 < 100
        {
        score_player_4 = score_player_4;
        }

        if score_player_1 > 100
        {
        score_player_1 = score_player_1 - dice_1;
        }

        else if score_player_2 > 100
        {
        score_player_2 = score_player_2 - dice_2;
        }

        else if score_player_3 > 100
        {
        score_player_3 = score_player_3 - dice_3;
        }

        else if score_player_4 > 100
        {
        score_player_4 = score_player_4 - dice_4;
        }

    turn += 1;

    println!("Press Enter!");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter);
  }
 }

  else
  {
    println!("Number of players allowed between 2-4.");
  }

 if score_player_1 == 100 || score_player_2 == 100 || score_player_3 == 100 || score_player_4 == 100
  {
    break;
  }

 }  
}

