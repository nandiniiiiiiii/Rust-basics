fn main() {
    println!("Hello, world!");
    // i8 - signed integer(both positive and negative values) and 8 bit number (max number - 2**7-1 and min number - -2**7 )
    // unsined int - non-negative values (0 and positive numbers)
    //u8 - if we know it's +ve by default
    let x: i8 = -5;      // by default it gives i32
    let y: u8 = 5; 
    let z: f32 = 100.03;
    print!("x: {}, y: {}, z: {}",x,y,z);
    //cant change without "mut"
    //let is_above: bool = true;

    //string- dont have fixed size at run time
    let ax: &str  = "hello how are you";
    print!("str: {ax}");

    let greeting: String = String::from("hello world");
    print!("string: {}",greeting);

    // @str are immutable,dont owns data,referes to string slice and can be stored any where while string are not, owns data, stored in heap

    // to print a single char in rust 
    let char1 = greeting.chars().nth(1);
    print!("char1 {}",char1.unwrap());

    let is_even: bool = true;
    if is_even{
        println!("is even");
    }
    else {
        println!("not");
    }// can also add else if

    for i in 0..10{
        println!("{}",i);
    }

    //array, maps, strings
    for _i in 0..10{
        println!("hello world")
    }

    let word: String = String::from("Hello how are you");
    let sentence = get_first_word(word);
    println!("sentence {}",sentence);

    create_str();
    let mut s1 = String::from("hello");
    s1 = take_ownership(s1);
    print!("{}",s1);

    let my_str: String = String::from("hello world");
    let s2: &String = &my_str;    //but this reference can not be chnaged(pushed)until we use &mut
    println!("{}",s2);

    s1.push_str(" World");
    println!("{}",s1);

    //updating ref using &mut   
    update_str(&mut s1);
    println!("{}",s1);

    //can have multiple borrower
    //let s1 = String::from("Hello");
    //let s2 = &s1;

    //one mut no borrowers
    //let mut s1: String = String::from("hello world");
    //let s2 = &mut s1;

}

fn update_str(s: &mut String){
    s.push_str(" how are you");
}

fn get_first_word(sentence: String) -> String{    // this fun return str
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}

//memory management 
//garbage collection: automatecly allocates and dealocates memory, Java, C#, Python, JavaScript, and Ruby.
//manual: needs prg is responsible for alocation and dealocation, languages like C and C++
//the rust way: Rust introduces a concept called ownership along with borrowing and lifetimes, which enables automatic and safe memory management at compile time without a garbage collector, makes it extreamly safe

//stack VS heap
//stack used: for premitive data types and data with fixed length- init intigers are pushed as a stack frame of a particular function
//heap used: data that can grow on run time(str/vector)

//cargo build rust->binary(compilation)

//pointrs rearly changes but when there is contiguious data then it do changes, in heap we need to allocate more data hence it's slower

//ownership- memory is managed via set of rules that the compiler checks, if any rules are violated prgg does't run
//the only way of cleanig data is when the str pointing tto that location dies and is replaced by another one
fn create_str(){
    let s1 : String =String::from("hello there");
    print!("{}",s1);
    let s2 = s1;
    print!("{}",s2);
    //to pass ownership we can use clone take_owenship(str.clone) or pass a mut str
}

fn take_ownership(some_str: String) -> String{
    print!("{}",some_str);
    return some_str;
}

//references- giving the address rather then the ownership
//let s1 = String::from("hello")
//let s2 = &s1;