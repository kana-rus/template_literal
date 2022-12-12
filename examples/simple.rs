use template_literal::tmp;

fn one() -> usize {1}

fn main() {
    let t = tmp!("result of one() is: ${one()}!!!");
    println!("{t}")
}