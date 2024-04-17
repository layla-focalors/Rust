fn main() {
    println!("Hello, world!");

    // 복합데이터 열거형 정의
    enum webEvent {
        // 열거된 형식, 데이터가 없는 경우
        WELoad,
        // 데이터 필드 키가 2개인 경우
        WEKeys(String, char);
        
        // 데이터 타입을 지정하는 경우
        WEClick { x:i64, y: i64 } 
    }

    // 구조체 기반 열거형 정의
    struct KeyPress(String, char);
    strust MouseClick {x:i64, y:i64}

    // 재정의
    enum webEvent {
        WELoad,
        WEClick(MouseClick),
        WEKeys(KeyPress);
    }

    // Veload (bool type);
    let we_load = webEvent::WELoad(true);

    // instance
    let click = MouseClick(x:100, y:250);
    let we_click = webEvent::WEClick(click);

    // 튜플 변형
    let keys = KeyPress(String::from("Ctrl +"), 'N');
    let We_key = webEvent::WEKeys(keys);
    
}
