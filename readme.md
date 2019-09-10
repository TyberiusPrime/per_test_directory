# `per_test_directory`

This crate introduces an attribute macro 
``` #[per_test_directory] ```
that will create a directory test\_runs/_module\_name_._test\_function\_name_
and change the current working directory to it while running.

If the test is successful, the directory is removed.
Otherwise, it is being kept for your inspection. 
Either way, the current directory is reset after the test run.


Example:
```rust	
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude;

    #[test]
    #[per_test_directory_macros]
    fn test_example() {
        let mut f = File::create("foo.txt")?;
        //actually in test_runs/tests.test_example/foo.txt
        file.write_all(b"hello");
        panic!("let's keep the file!");
    } }
```
