use std::cmp::Ordering;


// cargo add rand
// rand::thread_rng().gen_range(1..101);

fn main() {
    println!("猜一个1-100的数字");

    //生成一个随机数字，引用[dependencies] rand = "0.8.5" 来实现
    let secret_number = 3;

    //无限循环
    loop {
        println!("请输入一个猜测的数字: ");
        let mut guess = String::new();

        //读取命令行输入的内容、写入guess变量 &表示引用；mut表示可修改； expect表示出错时提示信息；
        std::io::stdin().read_line(&mut guess).expect("无法读取");
        //std::io::stdin().readline()读取其中的一行(输入流)

        //二次定义(遮了之后的)之后将guess变为i32类型、去除左右空格、转为i32类型
        //如果转换成功会将数值赋给新的guess变量，如果转换失败，提示输入不是数字，重新输入
        //trim()返回首尾空格与换行被删掉的String类型
        //parse()把string类型转化为需要的类型(如不能,返回Err)
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e)=>{//
                println!("你只能输入纯数字! {},{}",guess,e);
                continue;
            },
        };
        //switch语句，匹配输入数字是否正确
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("答对了!");
                break;
            },
            Ordering::Greater => println!("大了!"),
            Ordering::Less => println!("小了!"),

        }
    }
}
