use users::switch::set_current_uid;
use users::get_user_by_name;
use std::env;

fn main() -> Result<(), std::io::Error> {
    let switch_user = env::var("SWITCH_USER").unwrap_or("app".to_string());
    set_current_uid(get_user_by_name(switch_user.as_str()).unwrap().uid())?;
    unsafe { std::ptr::null_mut::<i32>().write(42) };
    Ok(())
}