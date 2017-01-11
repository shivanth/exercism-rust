#[derive(PartialEq,PartialOrd,Debug)]
pub struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    pub fn build(name: [i32;3])->Result<Triangle,&'static str> {
        if name[0] == 0 || name[1] ==0 || name[2]==0{
            return Err("Invalid Triangle")
        }
        if name[0] + name[1] < name[2] || name[0] + name[2] < name[1] || name[1] + name[2] < name[0] {
            return Err("Invalid Triangle")
        }
        Ok(Triangle {a : name[0],b : name[1],c : name[2]})
    }

    pub fn is_equilateral(&self) -> bool {
        if self.a == self.b && self.a == self.c {
            return true;
        }
        false
    }

    pub fn is_isosceles(&self) -> bool {
        if (self.a == self.b || self.a == self.c || self.b == self.c) && !self.is_equilateral(){
            return true;
        }
        false
    }
    pub fn is_scalene(&self) -> bool {
        !(self.is_isosceles() || self.is_equilateral())
    }
}
