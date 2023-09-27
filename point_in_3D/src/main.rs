

//定义trait
trait EulersDistance{
    fn euler_distance(p1:&Self,p2:&Self)->f64;
    type Output;
}
trait InnerProduct{
    fn in_pro(p1:&Self,p2:&Self) ->f64;
    type Output;
}


// 实现一个三维Point 结构体 impl 实现欧氏距离 实现一个new函数创建原点 实现一个点积函数 * p1 *p2 
#[derive(Debug)]//只对紧跟其后的Point起作用
struct Point{
    x:f64,
    y:f64,
    z:f64,
}
//实现trait
impl EulersDistance for Point{//trait的名字为EulersDistance
    type Output = f64;
    fn euler_distance(p1:&Self,p2:&Self)->f64{
        ((p1.x-p2.x)*(p1.x-p2.x)+(p1.y-p2.y)*(p1.y-p2.y)+(p1.z-p2.z)*(p1.z-p2.z)).sqrt()}
    
}
impl Point{
    fn new() -> Point{
       Point{x:0.,y:0.,z:0.}
    }
}
impl InnerProduct for Point{
    type Output = f64;
    fn in_pro(p1:&Self,p2:&Self) ->f64{
        p1.x*p2.x+p1.y*p2.y+p1.z*p2.z
    }
}
fn main(){
    let v1:Point=Point{x:1.0,y:2.0,z:3.0};//let mut v1:Point<f64,f64,f64> =Point{x:1.0,y:2.0,z:3.0};
    let v2:Point=Point{x:6.0,y:3.0,z:2.0};//上面注释用法仅当Point<T,T,T>(属性为泛型部分)时使用
    println!("v1与v2的欧氏距离为:{:?}",Point::euler_distance(&v1,&v2));
    println!("原点{:?}",Point::new());
    println!("内积{:?}",Point::in_pro(&v1,&v2))
}