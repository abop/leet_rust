use std::env;
use textplots::{Chart, Plot, Shape};

pub mod lifetime;
pub mod r#move;
pub mod n5_longest_palindrome_substring;
pub mod n6_zigzag_conversion;

fn main() {
    let args: Vec<String> = env::args().collect();
    // n5_longest_palindrome_substring::main();
    // n6_zigzag_conversion::main();
    // r#move::main();
    // lifetime::main();
    // let salary = Some(&args[1])
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .unwrap_or(65000);
    // let taxable_salary = Some(&args[2])
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .unwrap_or(36675);
    // calculat(salary, taxable_salary, Some(1000));
    for i in (3000..65000).step_by(1000) {
        println!("{:5} {:5}", i, taxed_income(i, Some(1000)) + (65000 - i) * 99 / 100);
    }
    // let points: Vec<(f32,f32)> = (3000..65000).step_by(1000).map(|i| 
    //     (i as f32, (taxed_income(i, Some(1000)) + (65000 - i) * 99 / 100)  as f32))
    //     .collect();
    //     // println!("{:?}", points);
    // Chart::new(80, 80, 3000.0, 65000.0).lineplot(&Shape::Lines(&points[..])).display();
}

fn calculat(monthly_salary: i32, taxable_salary: i32, deduction: Option<i32>) {
    let tax_free_salary = monthly_salary - taxable_salary;
    println!(
        "税后总收益：{}",
        taxed_income(taxable_salary, deduction) + tax_free_salary * 99 / 100
    );
    println!(
        "税后现金（缴税的部分）：{}",
        taxable_salary
            - tax(taxable_salary * 12, deduction)
            - gjj(monthly_salary)
            - sb(monthly_salary)
    );
    println!("免税薪资：{}", tax_free_salary);
    println!("个税：{}", tax(taxable_salary * 12, Some(1000)));
    println!("公积金：{}", gjj(taxable_salary));
    println!("社保：{}", sb(taxable_salary));
}

/// 计算税后收益
/// 社保属于长期收益，按100%税计算
fn taxed_income(monthly_salary: i32, deduction: Option<i32>) -> i32 {
    // monthly_salary - tax(monthly_salary * 12, deduction) + gjj(monthly_salary) - sb(monthly_salary)
    monthly_salary - tax(monthly_salary * 5+55000*7, deduction) + gjj(monthly_salary) - sb(monthly_salary)
}

/// 计算公积金
fn gjj(monthly_salary: i32) -> i32 {
    // 2022最新数据
    // 146701/12*3 = 2021年全市非私营单位就业人员月平均工资的3倍 = 36675
    let max_base = 36675;
    // 缴存基数下限为政府公布执行的当地月最低工资标准，即杭州市区下限为2280元，桐庐县、淳安县、建德市下限为2070元
    let min_base = 2280;

    return if monthly_salary > max_base {
        max_base * 12 / 100
    } else if monthly_salary < min_base {
        min_base * 12 / 100
    } else {
        monthly_salary * 12 / 100
    };
}

// 计算社保
fn sb(monthly_salary: i32) -> i32 {
    // 2021年数据，待更新
    // 79133/12*3 = 2020年浙江省非私营和私营单位就业人员加权月平均工资的3倍
    let max_base = 19783;
    // 月缴费基数下限暂按3957元（月平均工资的60%）
    let min_base = 3957;
    // 养老8%、失业0.5%、医保2%，共10.5%
    return if monthly_salary > max_base {
        max_base * 105 / 1000
    } else if monthly_salary < min_base {
        min_base * 105 / 1000
    } else {
        monthly_salary * 105 / 1000
    };
}

/// 每月的平均个税
///
/// * `deduction` - 特别抵扣，比如房贷利息
///
fn tax(annual_salary: i32, deduction: Option<i32>) -> i32 {
    // 5000元(免征额)
    let free_amount = 5000;
    let monthly_salary = annual_salary / 12;
    let monthly_taxable_amount = monthly_salary
        - free_amount
        - sb(monthly_salary)
        - gjj(monthly_salary)
        - deduction.unwrap_or(0);
    let annual_taxable_amount = monthly_taxable_amount * 12;
    /*
    级数 全年应纳税所得额 税率（%）速算扣除数
    1 不超过 36000 元的 3 0
    2 超过 36000 元至 144000 元的 10 2520
    3 超过 144000 元至 300000 元的 20 16920
    4 超过 300000 元至 420000 元的 25 31920
    5 超过 420000 元至 660000 元的 30 52920
    6 超过 660000 元至 960000 元的 35 85920
    7 超过 960000 元的 45 181920
    */
    let annual_tax = if annual_taxable_amount <= 0 {
        0
    } else if annual_taxable_amount <= 36000 {
        annual_taxable_amount * 3 / 100
    } else if annual_taxable_amount <= 144000 {
        annual_taxable_amount * 10 / 100 - 2520
    } else if annual_taxable_amount <= 300000 {
        annual_taxable_amount * 20 / 100 - 16920
    } else if annual_taxable_amount <= 420000 {
        annual_taxable_amount * 25 / 100 - 31920
    } else if annual_taxable_amount <= 660000 {
        annual_taxable_amount * 30 / 100 - 52920
    } else if annual_taxable_amount <= 960000 {
        annual_taxable_amount * 35 / 100 - 85920
    } else {
        annual_taxable_amount * 45 / 100 - 181920
    };
    return annual_tax / 12;
}
