use std::thread;

fn main() {
    first_example();
    move_closure_example();
    return_value_from_closure();
}

fn first_example() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn move_closure_example() {
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers {
            println!("number {n}");
        }
    })
    .join()
    .unwrap();
}

fn return_value_from_closure() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}
