fn main(){
    // variable
    let a_number = 10;
    // or
    let b_number;
    b_number = 10;

    let a_word = "Ten";
    println!("변수 제어");
    println!("The number is {}", a_number);
    println!("The Word is {}", a_word);

    // + rust, let에서는 값을 바인딩한 후 값을 변경할 수 없음!
    // 단 mut 를 붙이면 변수 바인딩 변경이 가능

    let mut halo = 10192891;
    println!("mut 바인딩 코드 변경");
    println!("The halo code is {}", halo);
    
    halo = 1289182;
    println!("The halo code is {}", halo);

    // 변수 셰도잉

    println!("변수 셰도잉, 이전의 기존 변수 이름을 사용하는 새 변수 생성");
    let shadow = 5;
    let shadow = shadow + 5;
    let shadow = shadow * 10;
    println!("shadow ID {}",shadow);

}