// use std::env::{Args, args};
use clap::{ arg, Parser};

#[derive(Parser,Debug)]

struct Read{
    ///读入几个数,result
    #[arg(short,long,num_args = 1..)]//num_args = 1..表示输入多个
    numbers:Vec<f64>,

    ///从(sum / magnitude / variance) 中选择一个作为选项
    #[arg{short,long}]
    order: String,
}
fn main(){
    let a=Read::parse();
    let result = match a.order.as_str(){  //String不能直接匹配&str类型,match a.order.as_str()把order转化为str类型
        "sum" =>{Some(a.numbers.iter().sum()) },//此处&str类型的sum即使使用.to_String也不能匹配order的String类型
        "magnitude" => Some(a.numbers.iter().fold(0. , |s, x| s+x*x).sqrt()),//fold中先初始值,再关系式,需要用','隔开,且初始值需要区分0和0.,来和||中返回的f64作运算
        "variance" => {
            let ave : f64= a.numbers.iter().sum::<f64>()/(a.numbers.len() as f64);//ave需要显式声明//a.numbers.len()返回usize类型,需要转化为f64才能参与运算
            Some(a.numbers.iter().fold(0. ,|s , x| s+(x-ave)*(x-ave)/a.numbers.len() as f64))
        },
        _=>{None}//由于可能输入的order不是以上三种,需要使用Option类型,把其他的命令打包返回None
    };
    println!("{:?}",result)
}