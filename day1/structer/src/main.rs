fn main() {
    struct Student { name: String, level:u8, remote:bool }

    struct Grades(char, char, char, char, f32);
    struct Unit;


    // 구조체 인스턴스화
    // 구조체 타입
    // 1. 구조체 지정, 2. 데이터 타입 할당, 3. 데이터 인스턴스유닛 선언 4. 데이터 입력
    let user_1 = Student { name: String::from("Constance Sharma"), remote:true, level:2};
    
    let user_2 = Student { name: String::from("Enwol Seo"), level:99, remote:false};

    let mark1 = Grades('a','a','a','a', 4.5);
    let mark2 = Grades('a','a','a','a', 4.5);

    println!("{}, level {}. remote: {}. Grades: {}, {}, {}, {}. Average: {}",user_1.name, user_1.level, user_1.remote, mark1.0, mark1.1, mark1.2, mark1.3, mark1.4);

    println!("{}, level {}. remote: {}. Grades: {}, {}, {}, {}. Average: {}",user_2.name, user_2.level, user_2.remote, mark2.0, mark2.1, mark2.2, mark2.3, mark2.4);
}
