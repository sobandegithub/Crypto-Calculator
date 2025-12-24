//Crypto Calculator
use std::io;

//convert SOL to lamports
fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

//convert lamports to SOL
fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

//calculate transaction fee: ~5000 lamports per signature

fn calculate_fee(num_signatures: u32) -> u64 {
    num_signatures as u64 * 5000
}

fn main() {
    println!("=== Crypto Calculator ===");
    println!("1. Convert SOL to Lamports");
    println!("2. Convert Lamports to SOL");
    println!("3. Calculate Transaction fee");
    println!("Enter your choice (1-3):");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    if choice == 1 {
        println!("Enter amount in SOL:");
        let mut sol_input = String::new();
        io::stdin()
            .read_line(&mut sol_input)
            .expect("Failed to read input");
        let sol: f64 = sol_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        let lamports = sol_to_lamports(sol);
        println!("{} SOL = {} lamports", sol, lamports);
    } else if choice == 2 {
        println!("Enter amount in lamports:");
        let mut lamports_input = String::new();
        io::stdin()
            .read_line(&mut lamports_input)
            .expect("Failed to read input");
        let lamports: u64 = lamports_input
            .trim()
            .parse()
            .expect("Please enter a valid number");
        let sol = lamports_to_sol(lamports);
        println!("{} lamports = {} SOL", lamports, sol);
    } else if choice == 3 {
        println!("Enter number of signatures:");
        let mut sig_input = String::new();
        io::stdin()
            .read_line(&mut sig_input)
            .expect("Failed to read input");
        let sigs: u32 = sig_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        let fee = calculate_fee(sigs);
        println!(
            "Transaction fee for {} signatures: {} lamports/ {} sol",
            sigs,
            fee,
            lamports_to_sol(fee)
        );
    } else {
        println!("Invalid choice!");
    }
}
