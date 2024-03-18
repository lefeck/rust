fn main() {
    let rect = Rectangle{width: 3, height: 8};
    println!("The area of the rectangle is {}", rect.area())

}

// struct tpye include metadata
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area ( &self ) ->u32 {
        /*
        在 Rust 中，分号（;）用于分隔表达式和语句，它告诉编译器单个表达式或语句已经结束。当用分号结束一个表达式时，它将变成一个语句，
        其值将被丢弃，并返回 ()，即单位类型。由于函数默认返回最后一个表达式的值，因此使用分号结束可能会导致不返回预期的值。
         */
        self.width * self.height
    }
}


