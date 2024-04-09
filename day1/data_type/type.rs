fn main(){
    // 32비트 정수로 값 할당
    let number: u32 = 14;
    let number2: u64 = 72;
    println!("The number is {}", number);
    println!("64bit {}", number2);


    // 부동소수점 데이터 형식
    let number_64 = 4.0;
    // f64 처리방식
    let number_32: f32 = 5.0;
    // f32 처리 방식

    // 붙여서 처리하기
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // boolean
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    let type1 = true;
    let type2 = false;
    if(type1 != type2){
        println!("Type1 and Type2 is diff");
    }else {
        println!("Type1 and Type2 is same");
    }
}