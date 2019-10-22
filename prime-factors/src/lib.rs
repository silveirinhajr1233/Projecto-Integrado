pub fn factors(mut n: u64) -> Vec<u64> {

	let mut v: Vec<u64> = Vec::new();
	let mut num = 2..;

	while n>1{

		let i = num.next().unwrap();

		while n%i==0{
			n /= i;
	   		v.push(i);
	    }  
	}     	    	
    
	
    return v;
}
