struct M{
    matrix:[[{integer};2];2],
}
impl M{
    fn add(&self,&other) ->Box<i32>{
    self.[[0,0],[0,1];[1,0],[1,1]]+other.[[0,0],[0,1];[1,0],[1,1]]
    }
}
fn main(){
    let m1= M{matrix:[[0,1];[2,3]]};
    let m2:M=M{matrix:[[2,8];[9,7]]};
    println!("add of m1 and m2 is {}",M(m1,m2));
}