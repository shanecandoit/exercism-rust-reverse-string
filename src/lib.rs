pub fn reverse(input: &str) -> String {
    
    let mut chs:Vec<char> = input.chars().collect();
    println!("chs {:?}", chs);
    
    chs.reverse();
    println!("chs2 {:?}", chs);

    //let s: String = input.into_iter().collect()

    let rev_str:String = chs.into_iter().collect();
    println!("rev_str {:?}", rev_str);

    rev_str

    /*
    let mut copy = String::with_capacity(input.len());
    for c in input.chars().rev() {
        //dbg!(c);
        copy.push(c);
    }

    let revd = String::from(copy);
    //println!("reverse {} = {}",input,revd);
    revd
    */
}

fn main() {
    println!("{}", reverse("cats"));
}