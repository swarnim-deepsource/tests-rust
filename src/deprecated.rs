fn foo() {
    //> [RS-W1128]: "The function `std::thread::sleep_ms` is deprecated"
    std::thread::sleep_ms(10);

    // fine
    std::thread::sleep(std::time::Duration::from_millis(10));

    //> [RS-W1128]: "The function `connect` is deprecated"
    let connected = ["hello", "world"].connect(" ");
}
