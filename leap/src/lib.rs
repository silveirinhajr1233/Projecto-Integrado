pub fn is_leap_year(year: u64) -> bool{
	
	match (year%4, year%100, year%400){
		(0,0,0) => return true,
		(0,0,_) => return false,
		(0,_,_) => return true,
		(_,_,_) => return false,
	}

	/*
	if year%4==0{
		if year%100==0{
			if year%400==0{
				return true;
			}
			return false;
		}
		return true;
	}
	return false;
	*/
}
