use proconio::input;
struct Cuboid {
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

impl Cuboid {
    fn intersects(&self, other: &Cuboid) -> bool {
        // X軸についての重なりをチェック
        if std::cmp::max(self.x1, other.x1) >= std::cmp::min(self.x2, other.x2) {
            return false;
        }
        
        // Y軸についての重なりをチェック
        if std::cmp::max(self.y1, other.y1) >= std::cmp::min(self.y2, other.y2) {
            return false;
        }
        
        // Z軸についての重なりをチェック
        if std::cmp::max(self.z1, other.z1) >= std::cmp::min(self.z2, other.z2) {
            return false;
        }
        
        // 全ての軸で重なりがある場合
        true
    }
}


fn main() {
    input!{
        a: (i32, i32, i32,i32,i32,i32),
        b: (i32, i32, i32,i32,i32,i32),
    }

    let c1 = Cuboid { x1: a.0, y1: a.1, z1: a.2, x2: a.3, y2: a.4, z2: a.5 };
    let c2 = Cuboid { x1: b.0, y1: b.1, z1: b.2, x2: b.3, y2: b.4, z2: b.5 };

    if c1.intersects(&c2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
