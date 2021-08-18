use graphe::directed;

enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

fn add(l: Expr, r: Expr) -> Expr {
    Expr::Add(Box::new(l), Box::new(r))
}

fn sub(l: Expr, r: Expr) -> Expr {
    Expr::Sub(Box::new(l), Box::new(r))
}

fn number(n: i64) -> Expr {
    Expr::Number(n)
}

// mod visitor {

//   visit_add()
//   visit_sub()
//   visit_number()

// }

fn main() {
    let expr = add(
        add(number(1), number(2)),
        sub(number(3), add(number(4), number(5))),
    );

    println!("Hello");
}
