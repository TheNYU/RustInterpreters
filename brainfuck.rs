use std::env;
use std::io;
use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

	let mut stack : [i8; 256] = [0; 256];
	let mut point = 0;
	let mut loops = Vec::with_capacity(256);

	if let Some(filename) = env::args().nth(1) {
		let path = Path::new(&filename);
		println!(">> Ouverture : {}", path.display());
		wait_for_input();
		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(w) => panic!("Impossible d'ouvrir le fichier : {}", w.description())
		};
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Impossible de lire le fichier");
		let args : Vec<_> = env::args().collect();
		if args.contains(&String::from("-c")) {
			println!(">> Contenu");
			println!("{}", content);
			wait_for_input();
		}
		println!(">> Exécution");
		content = content.replace("\r","").replace("\n"," ").replace(" ", "");
		let chars: Vec<_> = content.chars().collect();

		let mut i : usize = 0;
		let mut output = String::new();
		while i < chars.len() {
			match chars[i] {
				'>' => {
					if point == 255 {
						point = 0;
					} else {
						point += 1;
					}
				},
				'<' => {
					if point == 0 {
						point = 255;
					} else {
						point -= 1;
					}
				},
				'+' => {stack[point] += 1;},
				'-' => {stack[point] -= 1;},
				'[' => {loops.push(i);},
				']' => {
					if stack[point] == 0 {
						loops.pop();
					} else {
						i = loops.last().unwrap().to_owned();
					}
				},
				',' => {
					let mut input = String::new();
					print!("#? : ");
					io::stdout().flush().expect("Erreur : impossible de flush");
					io::stdin().read_line(&mut input).expect("Erreur");
					stack[point] = input.chars().nth(0).unwrap() as u8 as i8;
					if stack[point] == 13 {
						stack[point] = 0;
					}
				}
				'.' => {
					output.push(stack[point] as u8 as char);
					if stack[point] == 13 {
						output.push('\n');
					}
				},
				_ => {}
			}
			i += 1;
		}
		println!("\n>> Sortie");
		println!("{}", output);

	} else {
		println!("Rien du tout...");
	}
	wait_for_end();
	
}

fn wait_for_end() {
	println!("\nAppuyez sur Entrer pour quitter...");
	let mut _e = String::new();
	io::stdin().read_line(&mut _e).expect("Erreur");
}

fn wait_for_input() {
	let mut _e = String::new();
	io::stdin().read_line(&mut _e).expect("Erreur");
}
