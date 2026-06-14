use std::collections::HashMap;

fn main() {
    // 当 HashMap::new() 时，它并没有分配空间，容量为零
    let mut map = HashMap::new();
    explain("empty", &map);

    // 随着哈希表不断插入数据，它会以 2 的幂减一的方式增长，最小是 3
    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    // get 时需要使用引用，并且也返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    // get_key_value()，根据提供的 key 返回一个 (&k, &v) 格式的元组
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    // 删除后找不到了
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    // shrink 后 hash 变小，将容量裁剪到 len 大小
    map.shrink_to_fit();
    explain("shrinked", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{name}: len: {}, cap: {}", map.len(), map.capacity());
}
