#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    // Define your actual prompt ending
    const PROMPT_END: &str = " \x1b[33m$ \x1b[0m";
    
    // Helper function to create a temporary directory
    fn create_test_dir() -> PathBuf {
        let mut test_dir = env::current_dir().unwrap();
        test_dir.push("test_shell_temp");
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }

    // Helper function to clean up temporary directory
    fn cleanup_test_dir(test_dir: &PathBuf) {
        fs::remove_dir_all(test_dir).unwrap();
    }

    // Helper function to run shell with input commands
    fn run_shell_with_input(input: &str, test_dir: &PathBuf) -> (String, String) {
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(test_dir)
            .env("HOME", test_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn shell");

        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
        drop(stdin);

        let output = child.wait_with_output().expect("Failed to read output");

        (
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        )
    }

    #[test]
    fn test_echo_command() {
        let test_dir = create_test_dir();
        let (stdout, stderr) = run_shell_with_input("echo `pwd` `g` `v`\nexit\n", &test_dir);
        
        // Verify output contains expected text between prompts
        // assert!(stdout.contains("Command <echo hello world> not found"));
        println!("{stdout}");
        // assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_pwd_command() {
        let test_dir = create_test_dir();
        let (stdout, stderr) = run_shell_with_input("pwd\nexit\n", &test_dir);
        
        // Verify current path appears in output
        assert!(stdout.contains(&test_dir.display().to_string()));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_cd_command() {
        let test_dir = create_test_dir();
        let subdir = test_dir.join("test_cd");
        fs::create_dir(&subdir).unwrap();

        let input = format!("cd test_cd\npwd\nexit\n");
        let (stdout, stderr) = run_shell_with_input(&input, &test_dir);
        
        // Verify we see the subdirectory in output
        assert!(stdout.contains("test_cd"));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_command_substitution() {
        let test_dir = create_test_dir();
        let (stdout, stderr) = run_shell_with_input("echo $(echo nested)\nexit\n", &test_dir);
        assert!(stdout.contains("nested"));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_multi_line_quotes() {
        let test_dir = create_test_dir();
        let (stdout, stderr) = run_shell_with_input("echo \"line1\nline2\"\nexit\n", &test_dir);
        assert!(stdout.contains("line1"));
        assert!(stdout.contains("line2"));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_cat_command() {
        let test_dir = create_test_dir();
        let file_path = test_dir.join("test_file.txt");
        File::create(&file_path)
            .unwrap()
            .write_all(b"Hello\nWorld")
            .unwrap();

        let (stdout, stderr) = run_shell_with_input("cat test_file.txt\nexit\n", &test_dir);
        assert!(stdout.contains("Hello"));
        assert!(stdout.contains("World"));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_history_command() {
        let test_dir = create_test_dir();
        let input = "echo first\necho second\nhistory\nexit\n";
        let (stdout, stderr) = run_shell_with_input(input, &test_dir);
        assert!(stdout.contains("echo first"));
        assert!(stdout.contains("echo second"));
        assert!(stdout.contains("history"));
        assert_eq!(stderr, "");
        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_invalid_command() {
        let test_dir = create_test_dir();
        let (stdout, stderr) = run_shell_with_input("invalid_command\nexit\n", &test_dir);
        assert!(stderr.contains("Command <invalid_command> not found"));
        assert!(!stdout.contains("invalid_command"));
        cleanup_test_dir(&test_dir);
    }
}