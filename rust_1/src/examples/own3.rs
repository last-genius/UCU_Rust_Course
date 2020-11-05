fn om_nom_nom(s: String)
{    
    println!("I have consumed {}", s);
}

fn main() 
{    
    let s: String = "text".to_string();
    om_nom_nom(s);
    println!("{}", s);
}
