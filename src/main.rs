mod args;
mod diff;


fn main() {
    println!("positional: {}", args::parse_positional(1).unwrap());
    println!("flag: {}", args::parse_flag("del", Some("D")));
    println!("option: {}", args::parse_option("f", Some("F")).unwrap());

    println!("lcs: {}", diff::longest_common_subsequence(String::from("abcdef"), String::from("zabycdxwfd")));
}