fn om_nom_nom(n: u32) 
{    
    println!("{} is a very nice number", n);
}

fn main() 
{    
    let n: u32 = 110;
    let m = n;
    om_nom_nom(n);
    om_nom_nom(m);
    println!("{}", m + n);
}
