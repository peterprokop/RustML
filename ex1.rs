use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
                
fn main() {   
    match File::open("ex1data1.txt") {
    	Ok(file) => {
    		let mut array_2d = [[0.0f64; 3]; 200];
		    let mut buffer = BufReader::new(&file);

		    let mut x = 0;
		    for line in buffer.lines() {
		        let l = line.unwrap();
		        let pair: Vec<&str> = l.split(",").collect();

				array_2d[x][0] = 1.0;
				array_2d[x][1] = pair[0].parse::<f64>().unwrap();
				array_2d[x][2] = pair[1].parse::<f64>().unwrap();
		       		        
		        x += 1;
		    }  

		    for i in 0 .. 200 {
		    	let sub = array_2d[i];
		    	println!("{} {} {}", sub[0], sub[1], sub[2]);
		    }         
    	},
    	Err(e) => {
	        // fallback in case of failure.
	        // you could log the error, panic, or do anything else.
	        println!("{}", e);
    	}
	};

}   