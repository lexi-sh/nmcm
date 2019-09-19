pub mod bag;

fn main() {
    let cache = bag::Cache::new();
    cache.set("default", "key", "valueee");
    let v = cache.get("default", "key").unwrap();
    println!("{}", v);

}
