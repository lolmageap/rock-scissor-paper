use std::io;

fn main() {
    rock_scissor_paper();
    immutable_variable();
    mutable_variable();
    shadowing();
}


fn rock_scissor_paper() {
    println!("[가위, 바위, 보] 중 하나를 입력해주세요!");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("입력실패");
    println!("당신의 선택: {decision}");
 }
 
// 상수는 mut 못붙힘, 네이밍은 대문자로 작성
fn immutable_variable() {
    const PIE: f32 = 3.141592;
    println!("PIE 상수 값은 {PIE}입니다.")
}

// mut 키워드가 없다면 immutable 변수
fn mutable_variable() {
    let mut x = 3;
    println!("x의 값은 {x} 입니다.");
    x = 7;
    println!("x의 값은 {x} 입니다.");
}

// shadowing -> let은 변수를 다시 선언 가능(덮어 쓰기)
// 변수 내부에 스코프에서 변수를 또 선언 가능 -> 스코프를 벗어나면 덮어쓰기 전 값
fn shadowing() {
    let x = 3;
    println!("x의 값은 {x}입니다.");

    let x = x + 1;
    println!("x의 값은 {x}입니다.");
    {
        let x = x * 2;
        println!("안쪽 범위에서 x의 값은 {x}입니다.");
    }
    println!("x의 값은 {x}입니다.");
}