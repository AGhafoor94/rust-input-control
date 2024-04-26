use mouse_rs::{ types::keys::Keys, Mouse };
use std::process::{ Command };
// use std::io::{ Error };

fn main() 
{
    let mouse = Mouse::new();
    let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    mouse_control(&mouse);
    let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    println!("{:?}", (position_x, position_y));
    // execute_command("cmd", &["/C","echo hello"]);
    // execute_command("cmd", &["/C","start msedge"]);
    execute_command("cmd", &["/C","explorer https://www.google.co.uk"]);
}
fn mouse_control(mouse:&Mouse)
{
    mouse.move_to(500,500).expect("Unable to move");
    mouse.press(&Keys::RIGHT).expect("Unable to press");
    mouse.release(&Keys::RIGHT).expect("Unable to let go");
    mouse.move_to(0,0).expect("Unable to move");
}
fn get_mouse_position(mouse:&Mouse) -> (i32, i32)
{
    let position = mouse.get_position().unwrap();
    (position.x, position.y)
}
fn execute_command(exe: &str, args: &[&str])
{
    let command = Command::new(exe).args(&*args).output().expect("Can't run");
    println!("{:?}", command);
}