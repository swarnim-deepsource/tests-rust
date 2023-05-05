mod tests {
    use std::collections::HashSet;
    fn trivial() {
        let x = "foo";
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split("x");
        x.split("xx");
        x.split('x');
    }

    fn all_methods() {
        let x = "foo";
        //> [RS-P1100]: "Using single-character literal as pattern in `split_inclusive`, use a `char` instead"
        x.split_inclusive("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `contains`, use a `char` instead"
        x.contains("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `starts_with`, use a `char` instead"
        x.starts_with("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `ends_with`, use a `char` instead"
        x.ends_with("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `find`, use a `char` instead"
        x.find("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rfind`, use a `char` instead"
        x.rfind("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rsplit`, use a `char` instead"
        x.rsplit("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `split_terminator`, use a `char` instead"
        x.split_terminator("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rsplit_terminator`, use a `char` instead"
        x.rsplit_terminator("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `splitn`, use a `char` instead"
        x.splitn(3, "x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rsplitn`, use a `char` instead"
        x.rsplitn(3, "x");
        //> [RS-P1100]: "Using single-character literal as pattern in `split_once`, use a `char` instead"
        x.split_once("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rsplit_once`, use a `char` instead"
        x.rsplit_once("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `matches`, use a `char` instead"
        x.matches("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rmatches`, use a `char` instead"
        x.rmatches("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `match_indices`, use a `char` instead"
        x.match_indices("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `rmatch_indices`, use a `char` instead"
        x.rmatch_indices("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `trim_start_matches`, use a `char` instead"
        x.trim_start_matches("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `trim_end_matches`, use a `char` instead"
        x.trim_end_matches("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `strip_prefix`, use a `char` instead"
        x.strip_prefix("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `strip_suffix`, use a `char` instead"
        x.strip_suffix("x");
        //> [RS-P1100]: "Using single-character literal as pattern in `replace`, use a `char` instead"
        x.replace("x", "y");
        //> [RS-P1100]: "Using single-character literal as pattern in `replacen`, use a `char` instead"
        x.replacen("x", "y", 3);
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split("\t");
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split("'");
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split("\'");
    }

    fn no_match() {
        let h = HashSet::<String>::new();
        h.contains("X");
    }

    fn raw_strings() {
        let x = "foo";
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split(r"a");
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split(r#"a"#);
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split(r###"a"###);
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split(r###"'"###);
        //> [RS-P1100]: "Using single-character literal as pattern in `split`, use a `char` instead"
        x.split(r###"#"###);
    }
}
