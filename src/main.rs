fn main() {
    println!("0 is neither prime nor composite");
    println!("1 is a prime number");
    for count in 2..100 {
        let mut flag = 0;
        for divisor in 1..=count {
            if (count%divisor) == 0 {flag+=1;}
        }
        if flag==2 {println!("{} is a prime number", count);}
    }
}
// fn main() {
//     let slit = "so:me;str:ing;1:23;f:fd";
//     for word in slit.split(";") {
//         for entry in word.split(":"){println!("{}", entry);}
//         println!("{}", word);
//     // for i in slit {
//     //     println!("{:?}", i);
//     // }
//     }
// }
