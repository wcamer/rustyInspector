//use rand::Rng;

fn main(){
    println!("Welcome to Rust Inspector where you are going up against CPU in a classic game of hide and seek.  You and the CPU will take turns guessing where each other are on a XY grid and the first to be found loses.");
    println!("what is your name user?");
	let name = userSaid();
	

	
	println!("\nWelcome to the game of rusty investigator...{}",name);

	let mut status = "Seeking";
	println!("pick your hiding x");
	let hidx= userSaid().trim().parse::<i32>();//.expect("yo mama");
	println!("pick your hiding y");
	let hidy = userSaid().trim().parse::<i32>();//.expect("yo daddy");

	let mut p = Player{
		name: name,
		status: status.to_string(),
		hiding_spot_x: hidx.expect("Something went wrong with hiding_spot_x"),
  		hiding_spot_y:hidy.expect("something went wrong with hiding_spot_y"),
    		guess_x:0,
    		guess_y:0,
   		prev_x_guesses_vec: Vec::new(),
    		prev_y_guesses_vec: Vec::new(),
	
	};


	println!("here is the player object...{}", p.status);

	println!("Okay, {} your hiding place is set at {} {}.  Lets start the game!!!", p.name, p.hiding_spot_x, p.hiding_spot_y);

    //cpu creation
	let cname = "CPU";
	let mut cStat = "hiding";	
    //let mut rng = rand::thread_rng();
    // let cxhide = rng.gen_range(0..6);
    // let cyhide = rng.gen_range(0..6);
    let cxhide = 3; //needs to be updated
    let cyhide = 3; //needs to be updated to a random number

	let mut c = Player{
		name: cname.to_string(),
		status: cStat.to_string(),
		hiding_spot_x: cxhide,
  		hiding_spot_y: cyhide,
    	guess_x:0,
    	guess_y:0,
   		prev_x_guesses_vec: Vec::new(),
    	prev_y_guesses_vec: Vec::new(),
	};
    println!("here is the cpu name {} cpu xhide {} cpu yhide {} cpu status {}", c.name, c.hiding_spot_x, c.hiding_spot_y, c.status);
  

    move_finder(p , c);


}//end of the main function



struct Player{
    name:String,
    status:String,
    hiding_spot_x:i32, // this is for an integer
    hiding_spot_y:i32,
    guess_x:i32,
    guess_y:i32,
    prev_x_guesses_vec:Vec<i32>,
    prev_y_guesses_vec:Vec<i32>,
}//end of player sturct

fn userSaid() -> String{
    let mut response = String::new();
    let said = std::io::stdin().read_line(&mut response).unwrap().to_string();
    return response;
} // end of usersaid function

fn userSaidInt() -> i32{
    let mut response = String::new();
    std::io::stdin().read_line(&mut response);
    let num: i32 = response.trim().parse().expect("something went wrong with userSaidInt");
    return num
}



fn move_finder (person1: Player, person2: Player) {
    
    if person2.status == "Seeking" {
   
        println!("Watch out!!! ...{}!!!  It looks like {} is looking for you!!!", person1.name, person2.name);
        let rando_x = 5 ;
        let rando_y = 5;

        //to go in the check_positions 
        // person2.prev_x_guesses_vec.push(rando_x)
        // person2.prev_y_guesses_vec.push(rando_y)

        
       

          
        //if the guess is the same as the current guess the player is trying to make then they lose their turn
        // for i in person2.prev_x_guesses_vec.len() {
        //     if rando_x == person2.prev_x_guesses_vec[i] && rando_y == person2.prev_y_guesses_vec[i] {
        //         println!("CPU already guessed this spot at X = {} and Y = {}", rando_x,rando_y);
        //     }


            
        // }

        //check if the cpu found player or not
        check_positions(person1,person2,rando_x,rando_y);
       
        



    }
    //it is the user's turn
    else{
        println!("It is your turn... {}",person1.name);
        println!("Where do you want to guess for the x axis?");
        // let mut x_check:i32 = userSaid().parse().expect("something went wrong with x_check");// causes the code to panick here
        let mut x_check = userSaidInt();
        println!("Now enter a guess for the y axis");
        let mut y_check = userSaidInt();
        // let mut y_check:i32 = userSaid().parse().expect("something went wrong with y_check"); //


        // for i in person1.prev_x_guesses_vec.len() {
        //     if x_check == person2.prev_x_guesses_vec[i] && y_check == person1.prev_y_guesses_vec[i] {
        //         println!("You already guessed this spot at X = {} and Y = {}", x_check.parse().expect("something went wrong with x_check"), y_check.parse().expect("something went wrong with y_check"));
        //     }


            
        // }
    
    


        check_positions(person1, person2, x_check, y_check);




    }
   
}//end of move_finder


fn check_positions(mut person1: Player, mut person2: Player, x:i32, y:i32){
    if person1.status == "Seeking"{
        if person2.hiding_spot_x == x && person2.hiding_spot_y == y{
            println!("Congratulations {} YOU WON THE GAME!!!\nYOU WIN!!!\n\nGAMEOVER", person1.name);
        }else{
            person1.prev_x_guesses_vec.push(x);
            person1.prev_y_guesses_vec.push(y);
            println!("It looks like you haven't found the CPU yet...\nThe game continues...\n\n");
            person1.status = "Hiding".to_string();
            person2.status = "Seeking".to_string();
            move_finder(person1, person2)
        }
    } else{
        if person2.hiding_spot_x == x && person2.hiding_spot_y == y{
            println!("Oh no {}!!! The CPU found you!!!\nYou Lose...\n\nGAMEOVER", person1.name);
        }else{
            person2.prev_x_guesses_vec.push(x);
            person2.prev_y_guesses_vec.push(y);
            println!("It looks like you are still safe...\nThe game continues...\n\n");
            person2.status = "Hiding".to_string();
            person1.status = "Seeking".to_string();
            move_finder(person1,person2)
        }
    }
}//end of check positions
