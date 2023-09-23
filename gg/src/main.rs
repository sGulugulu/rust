#[derive(Debug)]//可以print list,自动输出所有能输出的变量值  但debug只对没有泛型的情况使用(本例子中使用了泛型,故可以不起作用)
enum List<T>{
    Cons(T,Box<List<T>>),
    Nil
}
use List::{Cons,Nil};//可以在之后的main函数中使用Cons与Nil
fn main(){
    let a=Cons(1,Box::new(List::Cons(3,Box::new(Nil))));
    let b=Cons(0,Box::new(a));
    let c=Cons(3,Box::new(b));
    match c {//由于a的所有权给了b,b的给了c,所以只能调用c
        Cons(x,_) => {
            println!("value of cons is {}",x);
        }
        _ => {}//_即others,其他可能
    }//判断c是否为Cons形式
}