pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
   
    return (1..limit)
    			.filter(|x| 
    			factors.iter()
    			.any(|y| *y!=0 && x%y==0))
    			.sum();									// sum dos valores que passam as condiçoes anteriores

}