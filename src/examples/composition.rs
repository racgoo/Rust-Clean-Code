// Rust는 Class가 없음 ( OOP에서 상속 개념이 존재하지 않음 )
// 즉 trait를 interface로 활용할 예정

use crate::examples;

//Bird 인터페이스 제작
trait Bird {
    fn can_fly(&self) -> bool;
    fn has_beak(&self) -> bool;
}


//Bird로 만들 CommonBird Class 훙내
struct CommonBird {}
impl Bird for CommonBird {
    fn can_fly(&self) -> bool {
        true
    }
    fn has_beak(&self) -> bool {
        true
     }
}

//CommonBird를 상속하지 않고 내부에 참조하는 형태의 컴포지션 진행
struct Chicken {
    bird: CommonBird,
}

//생성자 추가
impl Chicken {
    pub fn new() -> Chicken {
        Chicken {
            bird: CommonBird {},
        }
    }
}

//Bird의 기본 속성 추가
impl Bird for Chicken {
     fn can_fly(&self) -> bool {
        self.bird.can_fly()
    }
    fn has_beak(&self) -> bool {
        self.bird.has_beak()
    }
}

pub fn composition_test() {
    let chicken_instance = Chicken::new();
    println!("Chicken instance created");
    println!("Chicken Can Fly? : {}",chicken_instance.can_fly());
    println!("Chiken Can Beak? : {}",chicken_instance.has_beak());
}