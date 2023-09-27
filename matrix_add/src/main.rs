use std::ops::Add;
#[derive(Debug)]
struct M{
    matrix:[[i32;2];2],
}
impl Add for M {
    type Output = M;//输出M类型,对编译器声明,不占内存,需要自己指定其类型,
    fn add(self, other: Self) -> Self {//other的类型与self相同,都是M
        Self {
            // matrix: self.matrix[0][0]+other.matrix[0][0]+self.matrix[0][1]+other.matrix[0][1]+
            //self.matrix[1][0]+other.matrix[1][0]+self.matrix[1][1]+other.matrix[1][1];把一个数赋值给了self ,错误
            matrix:[[self.matrix[0][0]+other.matrix[0][0],
                    self.matrix[0][1]+other.matrix[0][1]],
                     [self.matrix[1][0]+other.matrix[1][0],
                     self.matrix[1][1]+other.matrix[1][1]]],
        }
    }
}
fn main(){
    let m1= M{matrix:[[0,1],[2,3]]};
    let m2= M{matrix:[[2,8],[9,7]]};
    let add=m1+m2;
    println!("add of m1 and m2 is {:?}",add);
}