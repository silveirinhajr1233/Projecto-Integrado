pub fn square_of_sum(n: u32) -> u32 {
	
	return (1..=n).fold(0, |x, n| x+n).pow(2);
	/*
    for i in 1..= n{
    	x=x+i;
    }
    */
    //return x*x;
}

pub fn sum_of_squares(n: u32) -> u32 {
	/*let mut y: u32 = 0;
	for i in 1..= n{	
		y=y+(i*i);
	}
	return y;*/

	return (1..=n).fold(0, |x, n| x+n.pow(2));

}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n)-sum_of_squares(n);
}
