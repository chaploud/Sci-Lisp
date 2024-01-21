use assert_cmd::prelude::*;
use assert_cmd::Command as AssertCmd;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn show_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A Lisp for Scientific Computation written in Rust"));
    Ok(())
}

#[test]
fn execute_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("tests/execute.lisp");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello from Sci-Lisp! [2024, 2024]"));
    Ok(())
}

#[test]
fn execute_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scilisp")?;
    cmd.arg("tests/notexist.rs");
    cmd.assert().failure().stderr(predicate::str::contains("IO Error"));
    Ok(())
}

// REPL tests

// literal
#[test]
fn execute_repl_00001() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r#"
        "abc\n"
        "#,
    );
    let out = "\"abc\\n\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00002() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        #"[0-9]+"
        "##,
    );
    let out = "#\"[0-9]+\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00003() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        false
        "##,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00004() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        true
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00005() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        nil
        "##,
    );
    let out = "nil";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00006() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        -987_654_321
        "##,
    );
    let out = "-987654321";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00007() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        -3.14e15
        "##,
    );
    let out = "-3140000000000000";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00008() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        nan
        "##,
    );
    let out = "NaN";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00009() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        inf
        "##,
    );
    let out = "inf";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00010() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        -inf
        "##,
    );
    let out = "-inf";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00011() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        -0.0
        "##,
    );
    let out = "-0";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00012() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        :keyword
        "##,
    );
    let out = ":keyword";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00013() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        'symbol
        "##,
    );
    let out = "symbol";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

// truthy/falsy
#[test]
fn execute_repl_00014() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if "" true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00015() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if '() true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00016() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if [] true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00017() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if {} true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00018() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if #{} true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00019() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if 0 true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00020() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if nan true false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00021() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if false true false)
        "##,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00022() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if nil true false)
        "##,
    );
    let out = "false";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

// Collection
#[test]
fn execute_repl_00023() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        '(1, "a", :b)
        "##,
    );
    let out = "(1 \"a\" :b)";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00024() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        [1.01, 2.01, 3.01]
        "##,
    );
    let out = "[1.01, 2.01, 3.01]";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00025() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        {:a "a", :b "b", :c "c"}
        "##,
    );
    let out = "{:a \"a\", :b \"b\", :c \"c\"}";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}
#[test]
fn execute_repl_00026() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        #{:a, :b, :c}
        "##,
    );
    let out = "#{:a, :b, :c}";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00027() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (type [1, 2, 3])
        "##,
    );
    let out = "\"vector\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00028() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (print [1, 2] "abc\n" 123)
        "##,
    );
    let out = "[1, 2] abc\n 123\nnil";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00029() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def a "abcde")
        a
        "##,
    );
    let out = "a\n\"abcde\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00030() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (const a "fghij")
        (def a "abcde")
        "##,
    );
    cmd.assert().success().stderr(predicate::str::contains("Const Error"));
    Ok(())
}

#[test]
fn execute_repl_00031() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def a "abcde")
        ([0|2] a)
        "##,
    );
    let out = "a\n\"ab\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00032() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (const C [1, 2, 3])
        (-1 C)
        "##,
    );
    let out = "C\n3";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00033() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (let [a 2]
          (set! a 3)
          a)
        "##,
    );
    let out = "3";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00034() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (defn sum [a b]
         "sum two value"
          (print a b)
          (+ a b))

        (sum 1 2)
        "##,
    );
    let out = "sum\n1 2\n3";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00035() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def sum
          (fn [a b]
            (return (+ a b))
            (+ a b)))

        (sum 1 2)
        "##,
    );
    let out = "sum\n3";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00036() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (if (< 2 3)
          true
          false)
        "##,
    );
    let out = "true";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00037() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (when (< 2 3)
           (do
             (print "2 < 3")
             "retval"))
        "##,
    );
    let out = "2 < 3\n\"retval\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00038() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def n 0)
        (cond
          (< n 0) "negative"
          (> n 0) "positive"
          :else "default")
        "##,
    );
    let out = "n\n\"default\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00039() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def val "c")
        (switch val
          ["a"]
            (print "A")
          ["b", "c"]
            (print "B or C")
          :default "DEFAULT")
        "##,
    );
    let out = "val\nB or C\nnil";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00040() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (for [i (range 5)]
          (print i))
        "##,
    );
    let out = "0\n1\n2\n3\n4\nnil";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00041() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (def a 0)
        (while (< a 10)
          (print a)
          (set! a (+ a 1))
          (if (> a 5)
            (break (+ a 9994))
            (continue))
          (print "never print"))
        "##,
    );
    let out = "a\n0\n1\n2\n3\n4\n5\n10000";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00042() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (inc 1)
        (dec 1)
        (+ 1 1)
        (- 1 1)
        (* 2 3)
        (/ 1 2)
        (// 5 2)
        (% 3 2)
        "##,
    );
    let out = "2\n0\n2\n0\n6\n0.5\n2\n1";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00043() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (= 2 2 2)
        (= 2 2 3)
        (!= 2 3)
        (< 2 3)
        (<= 3 3)
        (> 2 3)
        (>= 2 3)
        "##,
    );
    let out = "true\nfalse\ntrue\ntrue\ntrue\nfalse\nfalse";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00044() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (= 2 2 2)
        (= 2 2 3)
        (!= 2 3)
        (< 2 3)
        (<= 3 3)
        (> 2 3)
        (>= 2 3)
        "##,
    );
    let out = "true\nfalse\ntrue\ntrue\ntrue\nfalse\nfalse";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00045() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (nil? nil)
        (true? true)
        (false? false)
        (number? 1)
        (i64? 1)
        (f64? 1.0)
        (zero? 0)
        (even? 2)
        (odd? 3)
        (empty? [])
        (string? "abc")
        (keyword? :abc)
        (symbol? 'abc)
        (list? '(1, 2, 3))
        (vector? [1, 2, 3])
        (map? {:a 1, :b 2})
        (set? #{1, 2, 3})
        "##,
    );
    let out = "true\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00046() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (str 3.14)
        (str 'abc)
        (str :abc)
        (i64 "2")
        (f64 "3.14")
        (list #{1, 2, 3})
        (vector '(1, 2, 3))
        (vector "abc")
        (hmap [:a 1, :b 2])
        (hset [1, 2, 2])
        "##,
    );
    let out = "\"3.14\"\n\"abc\"\n\":abc\"\n2\n3.14\n(1 2 3)\n[1, 2, 3]\n[\"a\", \"b\", \"c\"]\n{:a 1, :b 2}\n#{1, 2}";

    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00047() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (sqrt 2)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("1.41421356"));
    Ok(())
}

#[test]
fn execute_repl_00048() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (abs -2)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("2"));
    Ok(())
}

#[test]
fn execute_repl_00049() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (cos (* 2.0 *pi*))
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("1"));
    Ok(())
}

#[test]
fn execute_repl_00050() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (sin (/ *pi* 2))
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("1"));
    Ok(())
}

#[test]
fn execute_repl_00051() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (tan 2.0)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("-2.1850398"));
    Ok(())
}

#[test]
fn execute_repl_00052() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (acos 0.5)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("1.04719"));
    Ok(())
}

#[test]
fn execute_repl_00053() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (asin 0.5)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("0.523598"));
    Ok(())
}
#[test]
fn execute_repl_00054() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (atan 0.5)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("0.463647"));
    Ok(())
}
#[test]
fn execute_repl_00055() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (log 2 10)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("3.321928"));
    Ok(())
}
#[test]
fn execute_repl_00056() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (ln *e*)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("1"));
    Ok(())
}
#[test]
fn execute_repl_00058() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (log10 2.0)
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("0.301029"));
    Ok(())
}
#[test]
fn execute_repl_00059() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (when (< 0 (rand) 1) "ok")
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("ok"));
    Ok(())
}

#[test]
fn execute_repl_00060() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (when (<= 5 (randint 5 30) 29) "ok")
        "##,
    );
    cmd.assert().success().stdout(predicate::str::contains("ok"));
    Ok(())
}

#[test]
fn execute_repl_00061() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (len "abcde")
        (len '(1, 2, 3))
        (len [1, 2, 3])
        (len #{1, 2, 3})
        (len {:a 1, :b 2, :c 3})
        "##,
    );
    let out = "5\n3\n3\n3\n3";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

#[test]
fn execute_repl_00062() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = AssertCmd::cargo_bin("scilisp")?;
    cmd.write_stdin(
        r##"
        (join [1, 2, 3] ",")
        "##,
    );
    let out = "\"1,2,3\"";
    cmd.assert().success().stdout(format!("{}\n", out));
    Ok(())
}

// type
// print
// doc
