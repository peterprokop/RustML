#![allow(non_snake_case)]

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
                

fn computeCost(X: [[f64; 2]; 97], y: [f64; 97], theta: [f64; 2]) -> f64 {
	let mut result = 0f64;
	let m = 97;
	for j in 0 .. m {
		//J = J + (theta(1) * X(j, 1) + theta(2) * X(j, 2) - y(j))^2;
		let tmp = theta[0] * X[j][0] + theta[1] * X[j][1] - y[j];
	    result += tmp * tmp;	
	}

	return result/(2f64 * (m as f64));
}

fn gradientDescent(x: [[f64; 2]; 97], y: [f64; 97], original_theta: [f64; 2], alpha: f64, iterations: u32) {
	let mut theta = original_theta;
	let m = 97; // number of training examples
	let n = 2;

	for _x in 0 .. iterations { 
	    let mut tmp_theta = [0f64; 2];
	    tmp_theta.clone_from_slice(&theta);

	 	for j in 0 .. n {
	 		let mut sum = 0f64;
	 		for i in 0 .. m { 
	            // sum = sum + (theta' * X(i, :)' - y(i)) * X(i, j);
	 			let tmp = theta[0] * x[i][0] + theta[1] * x[i][1];
	 			sum += (tmp - y[i]) * x[i][j];
	 		}
	        tmp_theta[j] -= sum * alpha / (m as f64); 

		}

	    theta = tmp_theta;
	    println!("theta {} {}", theta[0], theta[1]);
	    println!("computeCost: {}", computeCost(x, y, theta));
	}
}

fn main() {
    match File::open("ex1data1.txt") {
    	Ok(file) => {
    		let mut x = [[0.0f64; 2]; 97];
    		let mut y = [0.0f64; 97];
		    let mut buffer = BufReader::new(&file);
		    let mut lineNum = 0;

		    for line in buffer.lines() {
		        let l = line.unwrap();
		        let pair: Vec<&str> = l.split(",").collect();

				x[lineNum][0] 	= 1.0;
				x[lineNum][1] 	= pair[0].parse::<f64>().unwrap();
				y[lineNum] 		= pair[1].parse::<f64>().unwrap();
		       		        
		        lineNum += 1;
		    }  

		    // for i in 0 .. 200 {
		    // 	let sub = array_2d[i];
		    // 	println!("{} {} {}", sub[0], sub[1], sub[2]);
		    // }         
		    let iterations = 1500;
			let alpha = 0.01;
			let theta = [0f64; 2];
		    gradientDescent(x, y, theta, alpha, iterations)

    	},
    	Err(e) => {
	        // fallback in case of failure.
	        // you could log the error, panic, or do anything else.
	        println!("{}", e);
    	}
	};

}   