use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args:&[&str], expected_file:&str)->TestResult{
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("catr")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn fail(args:&[&str], expected:&str)->TestResult{
    Command::cargo_bin("catr")?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn skips_bad_file()->TestResult{
    fail(&["cant-touch-this"], "Permission denied")
}

#[test]
fn non_existent_file()->TestResult{
    fail(&["file1"], "No such file")
}

#[test]
fn empty()->TestResult{
    run(&["tests/inputs/empty.txt"], "tests/expected/empty.txt")
}

#[test]
fn emptyn()->TestResult{
    run(&["-n","tests/inputs/empty.txt"], "tests/expected/empty.n.txt")
}

#[test]
fn emptyb()->TestResult{
    run(&["-b","tests/inputs/empty.txt"], "tests/expected/empty.b.txt")
}


#[test]
fn fox()->TestResult{
    run(&["tests/inputs/fox.txt"], "tests/expected/fox.txt")
}

#[test]
fn foxn()->TestResult{
    run(&["-n","tests/inputs/fox.txt"], "tests/expected/fox.n.txt")
}

#[test]
fn foxb()->TestResult{
    run(&["-b","tests/inputs/fox.txt"], "tests/expected/fox.b.txt")
}


#[test]
fn spiders()->TestResult{
    run(&["tests/inputs/spiders.txt"], "tests/expected/spiders.txt")
}

#[test]
fn spidersn()->TestResult{
    run(&["-n","tests/inputs/spiders.txt"], "tests/expected/spiders.n.txt")
}

#[test]
fn spidersb()->TestResult{
    run(&["-b","tests/inputs/spiders.txt"], "tests/expected/spiders.b.txt")
}


#[test]
fn thebustle()->TestResult{
    run(&["tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.txt")
}

#[test]
fn thebustlen()->TestResult{
    run(&["-n","tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.n.txt")
}

#[test]
fn thebustleb()->TestResult{
    run(&["-b","tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.b.txt")
}


#[test]
fn all()->TestResult{
    run(&["tests/inputs/fox.txt","tests/inputs/spiders.txt","tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.txt")
}

#[test]
fn alln()->TestResult{
    run(&["-n","tests/inputs/fox.txt","tests/inputs/spiders.txt","tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.n.txt")
}

#[test]
fn allb()->TestResult{
    run(&["-b","tests/inputs/fox.txt","tests/inputs/spiders.txt","tests/inputs/the-bustle.txt"], "tests/expected/the-bustle.b.txt")
}
