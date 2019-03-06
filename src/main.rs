use std::io;

fn main() {
    println!("Hi. We're trying to figure out who we are. Can you help us? (Y/N)");
    let mut response = String::new();
    
   	io::stdin().read_line(&mut response)
   		.expect("error");

   	let checkresp = response.to_string();

   	let length = String::from(response).len();
   	
   	let yes = "Y".to_string();

   	if length > 2 {
   	 	println!("Sorry, we can't understand you.");
   	}
   	if length == 2 {
   		// uh I clearly don't understand how strings work
   		if checkresp == yes {
   			println!("Well, that's something.");
   		}
   		if checkresp != yes {
   			println!("That's too bad, because we're stuck in here together.");
   			println!("{}", checkresp);
   		}
   	}

}
