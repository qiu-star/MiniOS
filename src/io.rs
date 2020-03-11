use crate::sbi;
use core::fmt;

struct StdOut;

pub fn put_char(ch: usize)
{
    sbi::console_putchar(ch);
}

pub fn put_string(s: &str)
{
    for ch in s.bytes()
    {
        put_char(ch as usize);
    }
}

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    use core::fmt::Write;
    StdOut.write_fmt(args).unwrap();
}


impl fmt::Write for StdOut
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        put_string(s);
        Ok(())
    }
}