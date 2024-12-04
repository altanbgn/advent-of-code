pub fn printAnswer<T> (day: i32, section: &str, a: T)
where
    T: std::fmt::Debug
{
    println!("Day{}{} - Answer:{:?}", day, section, a);
}
