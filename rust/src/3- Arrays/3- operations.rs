fn initialization() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let mut dynamic_arr = Vec::new();
    dynamic_arr.push(1);
    dynamic_arr.push(2);
    dynamic_arr.push(3);
    dynamic_arr.push(4);
    dynamic_arr.push(5);
}

fn accessing_elements() {
    let arr = [1, 2, 3, 4, 5];

    println!("The first element is: {}", arr[0]);
    println!("The third element is: {}", arr[2]);
}

fn iteration() {
    let arr = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("{}", element);
    }
}

fn insertion() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let mut dynamic_arr = Vec::new();
    dynamic_arr.push(1);
    dynamic_arr.push(2);
    dynamic_arr.push(3);
    dynamic_arr.push(4);
    dynamic_arr.push(5);
}

fn deletion() {
    let mut arr = [1, 2, 3, 4, 5];

    // The following operation requires shifting all elements after the deletion point
    arr.remove(2);

    println!("Array after deletion: {:?}", arr);
}

fn search() {
    let arr = [1, 2, 3, 4, 5];

    // The following code performs a linear search through the array
    let mut found = false;
    for i in 0..arr.len() {
        if arr[i] == 3 {
            found = true;
            break;
        }
    }

    println!("Found: {}", found);
}
