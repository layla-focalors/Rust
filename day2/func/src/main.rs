fn main() {
    println!("Hello, world!");
    goodbye();
    let formal = "Formal: GoodBye.";
    let casual = "Casual : see you later";
    called_goodbye(formal);
    called_goodbye(casual);

    let num = 25;
    println!("{} devided by 5 = {}", num, divide_by_5(num));

    println!("{} devided by 5 = {}", num, return_based(num));
}

fn goodbye() {
    println!("Good Bye");
}

fn called_goodbye(message: &str){
    println!("\n{}", message)
}

// 값 반환,
// 일종의 리턴
fn divide_by_5(num: u32)  -> u32 {
    num / 5
}

// return
fn return_based(num: u32) -> u32 {
    if num == 0{
        return 0;
    }
    num / 5
}