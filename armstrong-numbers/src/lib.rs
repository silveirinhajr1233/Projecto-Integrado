pub fn is_armstrong_number(num: u32) -> bool {
    let mut x: [u32; 7] = [0;7];
	let mut cont =0;
	let mut y = num;
 
    loop{
    	
        x[cont] = y%10;
        cont +=1;
        if y<10 && y>0{
            break;
        }
        y/=10;
    }
    y=0;
    for i in x.iter().filter(|x| **x!=0){
        y += i.pow(cont as u32);
    }

    return y==num;
}
