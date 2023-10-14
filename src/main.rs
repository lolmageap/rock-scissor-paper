use std::{io, string};

fn main() {
    rock_scissor_paper();
    immutable_variable();
    mutable_variable();
    shadowing();
    print_number(99, 1);

    let radius = circle_radius(2.0);
    println!("반지름이 2.0인 원의 면적은 {radius}입니다.");
}


fn rock_scissor_paper() {
    println!("[가위, 바위, 보] 중 하나를 입력해주세요!");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("입력실패");
    println!("당신의 선택: {decision}");
 }
 
// 상수는 mut 못붙힘, 네이밍은 대문자로 작성
// const PIE: f32 = 3.141592;
const PIE: f64 = 3.141592;

fn immutable_variable() {
    // float은 기본 64 bit이고 f32를 명시하면 32 bit
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

fn scalar_type() {
    let add = 3 + 8;

    let sub = 26.5 - 2.1;

    let mul = 7 * 20;

    let quotient = 12.0 / 3.14;
    let truncate = 7 / 5; // 결과는 1

    let remainder = 46 % 5;

    let t = true;
    let f: bool = false;

    // char 타입은 유니코드로 이루어져 있기 때문에 이모지도 입력이 가능
    let c = 'A';
    let z: char = '가';
    let unicorn = '🦄';
}

// return type 이 없다면 unit 으로 반환
// tuple 은 여러 타입의 객체를 담을 수 있음
fn compound_type() {
    let tuple = (32, true, 1.41, "복합객체");
    // let tuple: (i32, bool, f64, str) = (32, true, 1.41, "복합객체");

    let (a, b, c, d) = tuple;

    let e = tuple.0;
    let f = tuple.1;
    let g = tuple.2;
    let h = tuple.3;


    println!("a는 {a}입니다.");
    println!("b는 {b}입니다.");
    println!("c는 {c}입니다.");
    println!("d는 {d}입니다.");
}

// 배열은 같은 타입만 담을 수 있음
// Rust에서 Array는 길이가 고정, 배열의 크기가 가변적일 경우 벡터를 사용해야함
fn array_type() {
    let x = [1, 2, 3, 4, 5];
    // let x: [i32, 5] = [1, 2, 3, 4, 5];

    let num = x[0];


    // 0번째 인덱스부터 99번째 인덱스까지 3이 총 100개 만들어짐
    let threes = [3; 100];

    let hellos = ["hello", 10];
    println!("{:?}", hellos);
}

// Rust는 snake case로 함수 네이밍
fn a_function() {
    println!("다른 함수입니다.")
}

fn print_number(a: i32, b: i32) {
    let sum = a + b;
    println!("a + b = {sum}");
}

// return 을 명시하지 않아도 스코프에 마지막 줄에 있는 겂아 리턴된다.
fn statement() {
    let x = 3;

// 식에서 리턴값은 세미콜론이 들어가면 안댐
    let y = {
        let x = 3;
        5 + x
    };

    println!(y); // 5
}

// 식에서 리턴값은 세미콜론이 들어가면 안댐
fn circle_radius(radius: f64) -> f64 {
    let r2 = radius * radius;
    PIE * r2
}

fn fn_if() {
    let x = 4;

    if x % 2 == 0 {
      println!("짝수");  
    } else {
      println!("홀수");  
    }

    let condition = true;
    let y = if condition {3} else {5};
    
    println!(y);
}

fn fn_loop() {
    // 무한 반복됨
    loop {
        println!("반복");
    }

    let mut counter = 0;

    let result = loop {
        println!("반복");
        counter += 1;
        if counter == 3 {
            break counter; // break 뒤에 반환 값을 넘길 수 있음
        }
    };
    
    println!(result);
    
    while (counter < 5) {
        println!("반복");
        counter += 1;
    }

    let arr = [1,2,3,4,5];
    let mut idx = 0;

    while idx < arr.len() {
        println!("array[idx] = {}", arr[idx]);
        idx += 1;
    }
    println!("완료");
    
    for value in arr {
        println!("value = {}", value);
    }
    println!("완료");

    for i in (0..5) {
        println!("value = {}", value);
    }
    println!("완료");

    // 거꾸로
    for i in (0..5).rev() {
        println!("value = {}", value);
    }
    println!("완료");

}