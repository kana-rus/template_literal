use template_literal::tmp;

fn one() -> usize {1}

fn main() {
    let a = tmp!("");
    let aa = tmp!("${}");
    let b = tmp!("result of one() is: ${one()}!!!");
    let c= format!("result of one() is: {}!!!", one());


}