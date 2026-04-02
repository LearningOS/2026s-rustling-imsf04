// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 修复：使用 if let 来解构 Option，语意极其精准
    // 意思是：“如果 option 里面真的有东西 (Some)，就把那个东西赋给 x，然后执行大括号里的逻辑。”
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}
