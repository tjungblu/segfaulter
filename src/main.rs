use std::env;
use users::get_user_by_name;
use users::switch::set_current_uid;

fn main() -> Result<(), std::io::Error> {
    for i in 1..1000 {
        println!("Logging a message {} from segfaulter", i);
    }
    let switch_user = env::var("SWITCH_USER").unwrap_or("app".to_string());
    set_current_uid(get_user_by_name(switch_user.as_str()).unwrap().uid())?;
    unsafe { std::ptr::null_mut::<i32>().write(42) };
    Ok(())
}
