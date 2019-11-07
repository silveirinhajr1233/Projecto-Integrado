pub fn raindrops(num: i32) -> String {

	let mut isto: String = num.to_string();
	
	if (num%3==0) && (num%5!=0) && (num%7!=0){ //por 3
		isto = String::from("Pling");
	}
	if (num%3!=0) && (num%5==0) && (num%7!=0){//por 5
		isto = String::from("Plang");
	}
	if (num%3!=0) && (num%5!=0) && (num%7==0){//por 7
		isto = String::from("Plong");
	}
		
	if ( num%3==0  && num%5==0) &&  !( num%3==0 &&  num%7==0) && !( num%5==0 &&  num%7==0){//por 3e5
		isto = String::from("PlingPlang");
	}
	if !( num%3==0 &&  num%5==0) &&  ( num%3==0 &&  num%7==0) && !( num%5==0 &&  num%7==0){//por 3e7
		isto = String::from("PlingPlong");
	}
	if !( num%3==0 &&  num%5==0) &&  !( num%3==0 &&  num%7==0) && ( num%5==0 &&  num%7==0){// por 5e7
		isto = String::from("PlangPlong")
	}
	
	if  num%3==0 &&  num%5==0 &&  num%7==0{ //por 3e5e7
		isto = String::from("PlingPlangPlong");
	}
		
	return isto;
}