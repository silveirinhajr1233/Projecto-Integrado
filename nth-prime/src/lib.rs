pub fn nth(n: u32) -> u32{

	let mut x: u32=0; //contador no numero de primos
	let mut num: u32=0; //quardar o numero primo
	let mut cont: u32;

	for i in 2..105000{
		
		cont = 0;
		for j in 2..i{
			
			if i!=j{
				if i%j==0{
					cont = cont +1;
				}
			}
			if cont >2{break;}	
		}
		if cont < 1 || i==2{
			
			num =i;
			x=x+1
		}
		
		if x>n {
			return num;
		}
	}
	return num;
} 
