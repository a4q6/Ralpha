use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

fn main() {
    let s = "exec_btc";
    let s_new = s.split("exec_").last().unwrap().replace("_", "");
    println!("{:?}", s_new);

    let mut map: BTreeMap<OrderedFloat<f64>, f64>;
    map = BTreeMap::new();
    map.insert(OrderedFloat(1.0), 1.0);
    map.insert(OrderedFloat(2.0), 2.0);
    println!("{:?}", map);
    println!("{:?}", map.last_key_value()); // 2, 2
    println!("{:?}", map.first_key_value()); // 1, 1
}
