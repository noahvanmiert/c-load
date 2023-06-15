use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    
    /* Create src folder */
    fs::create_dir("src-test").expect("Failed to create src/");
    
    /* create c source file */
    let mut file: File = File::create("src-test/main.c").expect("Failed to create src/main.c");

    let entry_point_template: &str = "#include <stdio.h>\n\nint main()\n{\n\tprintf(\"Hello, World!\");\n}\n";

    file.write_all(entry_point_template.as_bytes()).expect("Faild to write to src/main.c");
}
