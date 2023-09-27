fn main(){
    let mut a=vec![1,1];
    fibo(& mut a,5);
    println!("{:?}",a);
}
fn fibo(vec:& mut Vec<usize>,n:usize)->usize{
    let l:usize=vec.len();
    let a:usize = match l{
        l if l>=n-2 =>vec[n-2-1],
        _ =>fibo(vec,n-2),
    };
    let b:usize = match l{
        l if l>=n-1 => vec[n-1-1],
        _ => fibo(vec,n-1),
    };
    vec.push(a+b);
    a+b
}
// //兔子数列plus
// //f(n)=f(n-2)+f(n-2)+f(n-3)
// fn main (){
//     let mut a=vec![0,1,1];
//     fibo(& mut a,9);
//     println!("{:?}",a);
// }
// fn fibo(vec:& mut Vec<usize>,n:usize)->usize{
//     let a=match vec.len(){
//         l if l>=n-2 => vec[n-1-1],
//         _ =>fibo(vec,n-2),
//     };
//     let b= match vec.len(){
//         l if l>=n-3 =>vec[n-2-1],
//         _ => fibo(vec,n-3),
//     };
//     let c=match vec.len(){
//         l if l >=n-4 =>vec[n-3-1],
//         _ =>fibo(vec,n-4),
//     };
//     vec.push(a+b+c);
//     a+b+c
// }
