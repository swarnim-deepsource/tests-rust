#[rustfmt::skip]
mod tests {
    fn std() {
        let m = std::sync::Mutex::new(());
        let rw = std::sync::RwLock::new(());
        //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = m.lock();
        //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = rw.read();
        //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = rw.write();
        //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = m.try_lock();
        //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = rw.try_read();
       //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = rw.try_write();
    }

    fn parking_lot() {
        use parking_lot::{lock_api::RawMutex, Mutex, RwLock};

        let p_m: Mutex<()> = Mutex::const_new(RawMutex::INIT, ());
       //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = p_m.lock();

        let p_m1 = Mutex::new(0);
       //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = p_m1.lock();

        let p_rw = RwLock::new(0);
       //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = p_rw.read();
       //> [RS-E1014]: "This statement immediately drops the lock"
        let _ = p_rw.write();
    }
}
