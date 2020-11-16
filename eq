warning: CRLF will be replaced by LF in rust-toolchain.
The file will have its original line endings in your working directory
[1mdiff --git a/makefile b/makefile[m
[1mindex 840c509..1130b75 100644[m
[1m--- a/makefile[m
[1m+++ b/makefile[m
[36m@@ -26,4 +26,12 @@[m [mpublish: fmt[m
 	$(c) publish -v[m
 [m
 .PHONY: commit[m
[31m-commit: fmt test run[m
\ No newline at end of file[m
[32m+[m[32mcommit: fmt test run[m
[32m+[m
[32m+[m[32m.PHONY: cov[m
[32m+[m[32mcov: fmt[m
[32m+[m	[32mexport CARGO_INCREMENTAL=0[m
[32m+[m	[32mexport RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"[m
[32m+[m	[32mexport RUSTDOCFLAGS="-Cpanic=abort"[m
[32m+[m	[32m$(c) test[m
[32m+[m	[32mgrcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/[m
\ No newline at end of file[m
[1mdiff --git a/rust-toolchain b/rust-toolchain[m
[1mindex 870bbe4..687a15b 100644[m
[1m--- a/rust-toolchain[m
[1m+++ b/rust-toolchain[m
[36m@@ -1 +1,2 @@[m
[31m-stable[m
\ No newline at end of file[m
[32m+[m[32mnightly-x86_64-pc-windows-gnu[m
[32m+[m[32mstable-x86_64-pc-windows-gnu[m
\ No newline at end of file[m
[1mdiff --git a/src/util.rs b/src/util.rs[m
[1mindex 0e5d287..3f47623 100644[m
[1m--- a/src/util.rs[m
[1m+++ b/src/util.rs[m
[36m@@ -115,5 +115,5 @@[m [mmod tests {[m
         let mut v2 = vec_str_to_string(v1);[m
         assert_eq!(v2.pop(), Some(String::from(two)));[m
         assert_eq!(v2.pop(), Some(String::from(one)));[m
[31m-    }[m
[32m+[m	[32m}[m
 }[m
