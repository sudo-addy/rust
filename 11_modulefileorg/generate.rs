use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("generated.rs")?;
    writeln!(file, "// This file contains 10,000 lines of Rust code")?;
    writeln!(file)?;

    for i in 0..624 {
        writeln!(file, "struct S{} {{", i)?;
        writeln!(file, "    value: u32,")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(file, "impl S{} {{", i)?;
        writeln!(file, "    fn new(value: u32) -> Self {{")?;
        writeln!(file, "        S{} {{ value }}", i)?;
        writeln!(file, "    }}")?;
        writeln!(file, "    fn get(&self) -> u32 {{")?;
        writeln!(file, "        self.value")?;
        writeln!(file, "    }}")?;
        writeln!(file, "    fn set(&mut self, value: u32) {{")?;
        writeln!(file, "        self.value = value;")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    }

    writeln!(file, "fn main() {{")?;
    writeln!(file, "    println!(\"Hello from 10k lines!\");")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "#[cfg(test)]")?;
    writeln!(file, "mod tests {{")?;
    writeln!(file, "    use super::*;")?;
    writeln!(file)?;
    writeln!(file, "    #[test]")?;
    writeln!(file, "    fn test_structs() {{")?;
    writeln!(file, "        let s = S0::new(42);")?;
    writeln!(file, "        assert_eq!(s.get(), 42);")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}
