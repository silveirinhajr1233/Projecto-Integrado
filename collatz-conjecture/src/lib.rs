pub fn collatz(n: u64) -> Option<u64> {
	let mut cont =0;
	let mut x = n;
	if n==0{
		return None
	}
    while x!=1{
    	if x%2==0{
    		x=x/2;
    	}
    	else{
    		x= x*3+1;
    	}
    	cont+=1;
    }
    return Some(cont)
}
