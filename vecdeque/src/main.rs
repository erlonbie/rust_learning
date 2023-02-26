use std::collections::VecDeque;

// fn main() {
//     let mut my_vec = VecDeque::from(vec![0; 600_000]);
//
//     for _ in 0..600_000 {
//         my_vec.pop_front();
//     }
// }

fn checking_remaining(input: &mut VecDeque<(&str, bool)>) {
    for item in input {
        if !item.1 {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().expect("popping back...");
    task_done.1 = true;
    input.push_front(task_done);
}

fn main() {
    let mut my_vecdeque = VecDeque::new();
    let my_vec = [
        "At vero eos et accusam et justo duo dolores et ea rebum",
        "no sea takimata sanctus est Lorem ipsum dolor sit amet",
        "Stet clita kasd gubergren",
    ];

    for item in my_vec {
        my_vecdeque.push_front((item, false));
    }

    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    checking_remaining(&mut my_vecdeque);

    for task in my_vecdeque {
        println!("{:?}", task);
    }
}
