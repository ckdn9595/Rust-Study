//mod boj_2979;
//mod struct_without_fields;

fn main() {
    let mut s = String::from("1");
    {
        let mut r1 = &mut s;
        r1 = String::from("2");
        println!("{}", r1);
    }
    println!("{}", s);

   
    //boj_2979::main();
    //struct_without_fields::main();
}
