use std::collections::HashMap;

fn main() {
    let v = vec![-1, 4, 2, 4, 3, 5];
    search_zn(v);
}


fn search_zn(mut v: Vec<i32>) {
    let mut count: i32 = 0;
    let mut summ = 0;
    for i in &v {
        summ += i;
        count+=1;
    }
    let avarage = summ / count;

    v.sort();
    let mediana: i32;
    let i = count/2;
    if count%2==0 {
        mediana = (v[i as usize]+v[i as usize +1])/2;
    }
    else {
        mediana = v[i as usize]
    }

    let mut map = HashMap::new();
    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_value = 0;
    let mut max_key: i32 = 0;
    for (key, value) in &map {
        if *value>max_value{
            max_value = *value;
            max_key = *key;
        }
    }
    println!("Среднее значение: {},", avarage);
    println!("Медиана: {}", mediana);
    println!("{map:?}");
    println!("Мода: {}", max_key);

}
