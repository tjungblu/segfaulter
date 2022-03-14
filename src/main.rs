use std::env;
use users::get_user_by_name;
use users::switch::set_current_uid;
use std::alloc::{alloc, dealloc, Layout};
use std::{thread, time};
use std::mem;

fn malloc<T : Copy> (count: usize) -> *mut T
{
   let mut vec = Vec::<T>::with_capacity(count);
   let ret = vec.as_mut_ptr();
   mem::forget(vec); // avoid dropping the memory
   ret
}

fn main() -> Result<(), std::io::Error> {
    for i in 1..1000 {
        println!("Logging a message {} from segfaulter", i);
    }

    println!("Allocating 6GB of memory...");
    const BUF_LEN: usize = 1024 * 1024 * 1024 * 6;
    let buf = malloc::<u8>(BUF_LEN);
    dbg!(buf);

    // ensure the zero pages allocated are actually being written and pagefaulted properly
    unsafe {
        for n in 0..BUF_LEN {        
            *buf.add(n) = u8::from(42);        
        }
    }

    println!("waiting for crash...");
    let dur = time::Duration::from_secs(10);
    thread::sleep(dur);

    let switch_user = env::var("SWITCH_USER").unwrap_or("app".to_string());
    set_current_uid(get_user_by_name(switch_user.as_str()).unwrap().uid())?;
    unsafe { std::ptr::null_mut::<i32>().write(42) };
    Ok(())
}
