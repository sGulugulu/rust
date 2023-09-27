// ciw change inner word 
// caw change a word (swallow a space)
//cfo delete to next "o" (include o)change find o
//cto change till o
//BWbw
//x delete
//d <n> j 
//c <n> j
//ds'('     delete surronding '()'
//'cs'('+'{}'   change surrounding 先选中()再更改成{}
//r(字符)把当前光标替换
use std::ops::Add;//定义trait
use std::ops::Mul;//
#[derive(Debug,Clone)]
struct Complex{
    imag:f32,
    real:f32,
}
impl Add for Complex {//实现trait
    type Output = Complex;//输出M类型
    fn add(self, other : Self) -> Self {//other的类型与self相同,都是M
        Self {
        imag:self.imag+other.imag,
        real:self.real+other.real,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output { //继承output类型
        Self{
            imag:self.real*rhs.imag+self.imag*rhs.real,
            real:self.real*rhs.real-self.imag*rhs.imag,
        }
        
    }  
}
fn main(){
    let a= Complex{imag:4.,real:3.};
    let b= Complex{imag:5.,real:6.};
    let add=a.clone()+b.clone();
    let mul=a.clone()*b.clone();

    println!("add of a and b is {:?}",add);
    println!("multiplication of a and b is {:?}",mul);
    
}