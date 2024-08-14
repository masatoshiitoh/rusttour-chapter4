struct Person {name: Option<String>, birth: i32}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525});
    //let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    println!("composers[0].name is {}",
             match first_name {
                Some(name) => name,
                None => "none".to_string()
             });
    assert_eq!(composers[0].name, None);

    let x = vec![10,20,30];
    f(x);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let fifth = v.pop().expect("vector empty!");
    eprintln!("v contains: {:?}", v);
    eprintln!("fifth contains: {}", fifth);
    //let third = v[2];

    let mut v2 = Vec::new();
    for i in 101..106 {
        v2.push(i.to_string());
    }
    let second = v2.swap_remove(1);
    eprintln!("v2 contains: {:?}", v2);
    eprintln!("second contains: {}", second);

    let mut v3 = Vec::new();
    for i in 101..106 {
        v3.push(i.to_string());
    }
    let third = std::mem::replace(&mut v3[2], "substitute".to_string());
    eprintln!("v3 contains: {:?}", v3);
    eprintln!("second contains: {}", third);

    let v = vec!["librete".to_string(),
    "egalite".to_string(),
    "fraternite".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

fn f(x: Vec<i32>) {
    eprintln!("f received:{:?}", x);
}
