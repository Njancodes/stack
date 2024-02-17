use std::io;



fn push(arr: &mut [i32],max: &usize,top: &mut usize){
    if *top == *max{
        println!("The stack is full!!!");
    }else{
        let mut data = String::new();
        println!("Enter the data to be pushed into the stack");
        io::stdin().read_line(&mut data).expect("Something went wrong while entering data");
        let data:i32 = data.trim().parse().unwrap();

        arr[*top] = data;
        *top = *top + 1;
    }
}

fn pop(arr: &mut [i32],top: &mut usize){
    if *top == 0 && arr[*top] == 0{
        println!("The stack is empty");
    }else{
        *top = *top - 1;
        let data = arr[*top];
        arr[*top] = 0;
        println!("The data returned is {}.",data);
    }
}

fn display(arr: &mut [i32], max: &usize){
    for i in (0..*max).rev(){
        println!("{}",arr[i]);
    }
}

fn make_null(arr: &mut [i32],max: &usize,top: &mut usize){
    for i in 0..*max{
        arr[i] = 0;
    }
    *top = 0;
}

fn main() {
    let mut stack_arr: [i32;100] = [0;100];
    let mut max = String::new();
    let mut top = 0;

    println!("Enter the maximum number of elements in stack: ");
    io::stdin().read_line(&mut max).expect("The input for the max amount elements in stack has made some error");

    let max:usize = max.trim().parse().unwrap();
    let ch = 0;

    while ch != 5 {
        let mut ch = String::new();
        println!("Instructions:");
        println!("1. Push Elements");
        println!("2. Pop Elements");
        println!("3. Display");
        println!("4. Make Null");
        println!("5. Exit");
        println!("Enter the instruction: ");
        io::stdin().read_line(&mut ch).expect("Some error at the input of instruction");

        let ch:i32 = ch.trim().parse().unwrap();

        match ch{
            1 => push(&mut stack_arr,&max,&mut top),
            2 => pop(&mut stack_arr,&mut top),
            3 => display(&mut stack_arr,&max),
            4 => make_null(&mut stack_arr,&max,&mut top),
            5 => {println!("The program is going to exit now !!!!");break;},
            _ => println!("Enter a proper number for the instruction to execute")
        }
        
    }

}
