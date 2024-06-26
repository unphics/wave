fn main() {
    println!("Hello, world!");
    // 栈

    // 字符串常量"hello world"编译时被存入exe的.RODATA段(gcc)或.RDATA段(vc++),然后在程序加载时获得一个固定的地址.
    // 当执行"hello world".to_string()时, 在堆上, 一块新的内存被分配出来, 并把"hello world"逐个字节拷贝过去.
    // 把堆上的数据赋值给s, s作为分配在栈上的一个变量, 它需要知道堆上内存的地址, 另外由于堆上的数据大小不确定且可以增长, 还需要知道它的长度以及它现在有多大.
    // 最终为了表述这个字符串使用了3个word: 指针,字符串当前长度(11),这片内存的总容量(11). 64位系统下3个word24个字节
    let s = "hello world".to_string();

    // 堆
}
