use gdb_command::*;

/// Returns an absolute path for relative path.
fn abs_path<'a>(rpath: &'a str) -> String {
    use std::path::PathBuf;

    // Define paths.
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut path = PathBuf::new();
    path.push(&project_dir);
    path.push(rpath);

    path.as_os_str().to_str().unwrap().to_string()
}

#[test]
fn test_local_canary() {
    let mut args = Vec::new();
    let bin = abs_path("tests/bins/test_canary");
    let a = std::iter::repeat("A").take(200).collect::<String>();
    args.push(bin.as_str());
    args.push(a.as_str());
    let result = GdbCommand::new(&ExecType::Local(&args)).bt().run();
    if result.is_err() {
        assert!(false, "{}", result.err().unwrap());
    }
    let result = result.unwrap();
    assert_eq!(result[0].contains("__stack_chk_fail"), true);
}

#[test]
fn test_local_safe_func() {
    let mut args = Vec::new();
    let bin = abs_path("tests/bins/test_safeFunc");
    let a = std::iter::repeat("A").take(200).collect::<String>();
    args.push(bin.as_str());
    args.push(a.as_str());
    let result = GdbCommand::new(&ExecType::Local(&args)).bt().run();
    if result.is_err() {
        assert!(false, "{}", result.err().unwrap());
    }
    let result = result.unwrap();
    assert_eq!(result[0].contains("__strcpy_chk"), true);
}

#[test]
#[ignore] // To run this test: If Ubuntu 20.04 just remove ignore. Other systems: recollect the core.
fn test_core_canary() {
    let bin = abs_path("tests/bins/test_canary");
    let core = abs_path("tests/bins/core.test_canary");
    let result = GdbCommand::new(&ExecType::Core {
        target: &bin,
        core: &core,
    })
    .bt()
    .run();
    if result.is_err() {
        assert!(false, "{}", result.err().unwrap());
    }
    let result = result.unwrap();
    assert_eq!(result[0].contains("__stack_chk_fail"), true);
}

#[test] // To run this test: If Ubuntu 20.04 just remove ignore. Other systems: recollect the core.
#[ignore]
fn test_core_safe_func() {
    let bin = abs_path("tests/bins/test_safeFunc");
    let core = abs_path("tests/bins/core.test_safeFunc");
    let result = GdbCommand::new(&ExecType::Core {
        target: &bin,
        core: &core,
    })
    .bt()
    .run();
    if result.is_err() {
        assert!(false, "{}", result.err().unwrap());
    }
    let result = result.unwrap();
    assert_eq!(result[0].contains("__strcpy_chk"), true);
}

#[test] // To run this test: echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
#[ignore]
fn test_remote_unwind() {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    let mut child = Command::new(abs_path("tests/bins/test_callstack_remote"))
        .spawn()
        .expect("failed to execute child");
    thread::sleep(Duration::from_millis(10));

    let result = GdbCommand::new(&ExecType::Remote(&child.id().to_string()))
        .bt()
        .run();
    if result.is_err() {
        assert!(false, "{}", result.err().unwrap());
    }

    let result = result.unwrap();
    assert_eq!(result[0].contains("third"), true);
    assert_eq!(result[0].contains("second"), true);
    assert_eq!(result[0].contains("first"), true);
    assert_eq!(result[0].contains("main"), true);

    child.kill().unwrap();
}
