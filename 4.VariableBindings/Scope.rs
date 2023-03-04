fn main(){
    //This Biniding lives in the  main function
    let long_lived_binding = 1;

    //This is a Block and has a smaller scope than the main function
    {
        //This Binding exists only in this block
        let short_lived_binding = 2;
        println!("innner short:{}", short_lived_binding);
    }
    //end the block
    //println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}