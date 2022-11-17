fn main() {
    println!("Hello, world!");

    // println! 传入格式化字符串，自动推导占位符
    println!("One month has {} days and {} hours.", 31, 24 * 31);

    // println! 多个参数通过索引出现在格式化字符串中不同的位置
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // println! 格式化字符串中使用变量名
    println!("{a} {b} {c}.", b = 2, a = 1, c = 3);

    // println! 使用 : 定制格式化输出的格式
    println!("Base 10:                {}", 69420);
    println!("Base 2 (binary):        {:b}", 69420);
    println!("Base 8 (octal):         {:o}", 69420);
    println!("Base 16 (hex):          {:x}", 69420);
    println!("Base 16 (HEX):          {:X}", 69420);

    println!("一个块区域中，最后一个语句可以不用分号结尾")
}
