#[derive(Debug)]
enum TsvValue {
    Str(String),
    Num(f64),
    Null,
}
fn get_amount_desc(val: TsvValue) -> String {
    // match 返回String，赋值给desc
    let desc = match val {
        TsvValue::Num(n) if n > 100.0 => "大额金额".to_string(),
        TsvValue::Num(n) => format!("小额金额：{}", n),
        TsvValue::Str(s) => format!("非数字：{}", s),
        TsvValue::Null => "空金额".to_string(),
    };
    desc
}

fn main() {
    let val = TsvValue::Num(10.0);
    println!("{}", get_amount_desc(val)); // 大额金额
}