#[rustfmt::skip]
mod tests {
    fn main() {
        // ok
        let mut option = Some(1);
        let _ = Box::new(move || option.take().unwrap());

        // ok
        let x = Some(3);
        x.as_ref();

        // not ok
        let x = Some(3);
       //> [RS-W1083]: "Called `Option::take` on a temporary value"
        x.as_ref().take();
    }
}
