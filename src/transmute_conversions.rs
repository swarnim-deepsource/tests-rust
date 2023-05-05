mod test {
    use std::{rc::Rc, sync::Arc};

    fn trivial() {
        let val: u32 = 120;
        let ptr = &val as *const u32;
        unsafe {
            //> [RS-E1018]: "Transmuting from `ptr` to `ref` can be buggy, consider using `as`"
            let rf1: &u32 = std::mem::transmute(ptr);
            //> [RS-E1019]: "Transmuting from `ref` to `ptr` can be buggy, consider using `as`"
            let r2: *const u32 = std::mem::transmute(rf1);
            //> [RS-E1020]: "Transmuting from `ptr` to `ptr` can be buggy, consider directly transmuting between `refs` first."
            let r2: *const u32 = std::mem::transmute(rf1 as *const u32 as *const i32);
        }

        unsafe {
            //> [RS-E1024]: "Transmuting from `T` to `*T` or `&T` is likely a bug"
            let cpt1: &u64 = std::mem::transmute(val as u64);
            //> [RS-E1024]: "Transmuting from `T` to `*T` or `&T` is likely a bug"
            let cpt2: *const u64 = std::mem::transmute(val as u64);
            //> [RS-E1025]: "Transmuting from integer to `bool`, consider `0i8 == 0` instead"
            let tib: bool = std::mem::transmute(0i8);
            //> [RS-W1117]: "Transmuting from `T` to `T` is redundant"
            let ut1: u32 = std::mem::transmute(val);
            //> [RS-W1117]: "Transmuting from `T` to `T` is redundant"
            let ut2: &u64 = std::mem::transmute(cpt1);
            //> [RS-W1117]: "Transmuting from `T` to `T` is redundant"
            let ut3: *const u64 = std::mem::transmute(cpt2);
            //> [RS-W1118]: "Using `c_void` raw pointers in `Box::from_raw`"
            let frwvpt = Box::from_raw(1 as *mut std::ffi::c_void);
            //> [RS-W1118]: "Using `c_void` raw pointers in `Arc::from_raw`"
            let frwvpt = Arc::from_raw(1 as *const std::ffi::c_void);
            //> [RS-W1118]: "Using `c_void` raw pointers in `Rc::from_raw`"
            let frwvpt = Rc::from_raw(1 as *const std::ffi::c_void);
        }

        unsafe {
            //> [RS-E1026]: "Transmuting from integer to `NonZero` type"
            let nz1: std::num::NonZeroU32 = std::mem::transmute(val);
            //> [RS-E1027]: "Transmuting from int literal (possibly null) to `fn` ptr"
            let nz2: fn() -> () = std::mem::transmute(1 as *const u32);
            //> [RS-E1027]: "Transmuting from `std::ptr::null*` to `fn` ptr"
            let nz2: fn() -> () = std::mem::transmute(std::ptr::null::<u32>());
            //> [RS-E1028]: "Transmuting from literal (possibly null) to `*T` ptr"
            let nz2: *const u32 = std::mem::transmute(0u64);
            //> [RS-W1119]: "Manual implementation of `std::ptr::eq`"
            &10u32 as *const u32 == std::ptr::null();
        }

        unsafe {
            let x = 10.0f32;
            //> [RS-W1126]: "Transmuting float to a integer, consider `x.try_into().unwrap()` instead"
            let y: i32 = std::mem::transmute(x);
            //> [RS-W1127]: "Transmuting integer to a float, consider `y.try_into().unwrap()` instead"
            let x: f32 = std::mem::transmute(y);
            //> [RS-E1029]: "Transmutes from float to reference is invalid, and unsound"
            let rx: &f32 = std::mem::transmute(x as f64);
            //> [RS-E1024]; [RS-E1029]
            let px: *const f64 = std::mem::transmute(x as f64);
            //> [RS-E1029]: "Transmutes from float to pointer is invalid, and unsound"
            let px: *const i32 = std::mem::transmute(x as f64);
            let c = 'c';
            //> [RS-D1000]
            // TODO(swarnim): [RS-E1029]: "Transmutes from char to reference is invalid, and unsound"
            let rc: &i32 = std::mem::transmute(c as u64);
            //> [RS-D1000]
            // TODO(swarnim): [RS-E1029]: "Transmutes from char to pointer is invalid, and unsound"
            let pc: *const i32 = std::mem::transmute(c as u64);
            let x: u32 = 0x0000_0061;
            //> [RS-E1030]: "Transmuting integer to `char`, consider `char::from_u32(x.try_into()?)` instead."
            let c: char = std::mem::transmute(x);
            println!("{}", c);
            let x: u32 = 0x0000_D800;

            //> [RS-E1030]: "Transmuting integer to `char`, consider `char::from_u32(x.try_into()?)` instead."
            let c: char = std::mem::transmute(x);
            //> [RS-E1031]
            let zeroed_bytes: [u8; 8] = std::mem::transmute(0u64);
            //> [RS-E1032]
            let zeroed_array: [u64; 2] = std::mem::transmute((0u64, 0u64));
        }
    }
}
