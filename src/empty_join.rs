
#[rustfmt::skip]
fn f() {
     //> [RS-W1203]: "Using `.join(..)` with empty string literal, prefer `.concat()` instead"
     let m = ["test", "ing"].join("");
     let array = ["tt", "tt"];
     //> [RS-W1203]: "Using `.join(..)` with empty string literal, prefer `.concat()` instead"
     array.join("");
}

fn t(sl: &[&str]) -> String {
     //> [RS-W1203]: "Using `.join(..)` with empty string literal, prefer `.concat()` instead"
     sl.join("")
}

fn no_match() {
     let _ = ["test", "ing"].join(" ");
     let _ = ["test", "ing"].join("-");
     let _ = ["test", "ing"].concat();
     let a = ["test", "ing"];
     a.join("*");
}
