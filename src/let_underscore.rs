//> scatr-check: RS-W1129

async fn bar() -> i32 { 0 }

async fn foo() {
    //> [RS-E1035]: "This statement disposes of the `Future` directly, consider using await/poll on future to execute it"
    let _ = bar();
    //> [RS-E1035]: "This statement disposes of the `Future` directly, consider using await/poll on future to execute it"
    _ = bar();
    //> [RS-E1035]: "This statement disposes of the `Future` directly, consider using await/poll on future to execute it"
    let _ = async { bar().await };

    // fine!
    let _ = bar().await;
    let x = bar();

    //> [RS-W1129]: "This statement is ignoring a unit type, `let` binding is redundant"
    let _ = ();
    //> [RS-W1129]: "This statement is ignoring a unit type, `let` binding is redundant"
    let _ = std::thread::sleep_ms(10);
    //> [RS-W1129]: "This statement is ignoring a unit type, `let` binding is redundant"
    let _ = vec![].push(0);
}
