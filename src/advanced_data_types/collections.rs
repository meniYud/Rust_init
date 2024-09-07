/*
Source: https://www.youtube.com/watch?v=f4a8rWkSl8U&list=PLPoSdR46FgI412aItyJhj2bF66cudB6Qs&index=19


In this lesson, we dive deep into the world of Rust collections, focusing on Vectors.
We'll start by explaining what a Vector is and how to create one in Rust.

Next, we'll cover essential Vector methods like push and pop,
and demonstrate how to read elements by index and using the get method.

You'll also learn how to iterate over Vector values efficiently and how to store multiple types using enums.
Finally, we'll discuss the lifecycle of a Vector and what happens when it is dropped. 

*/

pub fn vectors(){
    let vec1: Vec<i32> = Vec::new();
    println!("Constructed vector: {:?}", vec1);

    let vec2 = vec![1,2,3,4,5,6];
    println!("Instantiated vector: {:?}", vec2);

    //update a vector
    let mut vec3: Vec<i32> = Vec::new();

    vec3.push(1);
    vec3.push(2);
    vec3.push(3);
    println!("updated vector: {:?}", vec3);

    let popped = vec3.pop();
    println!("Popped value is: {:?}\nVector value is now: {:?}", popped, vec3);

    //read vector content
    let mut vec4 = vec!['a', 'b', 'c', 'd', 'e'];

    // vec4.pop();
    // vec4.pop();
    // vec4.pop();

    // unsafe access. if vec4 is modified, might panic with "index out of bounds"
    let fourth: &char = &vec4[3];
    println!("The fourth element is: {}", fourth);

    // vec4.pop();

    //safe access
    let fifth: Option<&char> = vec4.get(4);
    match fifth {
        Some(&fifth) => println!("The fifth element is: {:?}", &fifth),
        None => println!("Vector doesn't hold fifth element, or fifth element is None")
    }

}