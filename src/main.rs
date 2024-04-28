use std::{ fs::{self, DirEntry, File, FileType}, io::{self, Read, Write}, path::Path, process::{ Command, Output }, str, time::Duration };
use mouse_rs::{ types::{keys::Keys, Point}, Mouse };
use windows::Win32::{
    UI::WindowsAndMessaging::*
};
use windows::core::{ s };
// use std::io::{ Error };

fn main() 
{
    let file_location:&str = "./test-file.txt";
    let mouse:Mouse = Mouse::new();
    let screen_res:Result<Output, std::io::Error> = execute_command("cmd", &["/C","wmic PATH Win32_VideoController GET CurrentVerticalResolution,CurrentHorizontalResolution"]);
    // let screen_res:Result<Output, std::io::Error> = execute_command("cmd", &["/C","wmic PATH Win32_VideoController GET SystemName"]);
    // execute_command("cmd",&["/C","start C:\\Users\\"]);
    // let screen_res:Output = execute_command("cmd",&["/C","wmic context"]);
    let mut screen_out:Output = screen_res.unwrap();
    let _ = copy_all_files_in_directory("C:/Users/adnan/Downloads/test source","C:/Users/adnan/Downloads/test");
    let std_out_values:Vec<u8> = std::mem::take(&mut screen_out.stdout);
    println!("{:?}",std_out_values);
    let file_status:Result<(), std::io::Error> = write_output_to_file(&std_out_values, file_location);
    match file_status {
        Ok(()) => println!("Successfully written file"),
        Err(e) => println!("Error writting file {}", e)
    };
    let value:&str = match str::from_utf8(&std_out_values){
        Ok(val) => val,
        Err(e) => panic!("Invalid {}",e)
    };
    println!("{:?}", &value.trim());
    let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    mouse_control(&mouse);
    let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    println!("{:?}", (position_x, position_y));
    // execute_command("cmd", &["/C","echo hello"]);
    // execute_command("cmd", &["/C","start msedge"]);
    // execute_command("cmd", &["/C","explorer https://www.office.com"]);
    let _ = press_hold_mouse(&mouse);
    let _ = move_mouse_to_location(&mouse, 20, 20);
    let _ = move_mouse_to_location(&mouse, 40, 20);
    std::thread::sleep(Duration::from_secs(5));
    let _ = release_mouse(&mouse);
    unsafe {
        // let title:PWSTR = [0b1010];
        // let caption:HSTRING = "World".into();
        MessageBoxA(None, s!("Caption"), s!("Title"), MB_OK);

    }
    
}
fn press_hold_mouse(mouse:&Mouse) -> Result<(), Box<dyn std::error::Error>>
{
    mouse.press(&Keys::LEFT)
}
fn move_mouse_to_location(mouse:&Mouse, x:i32, y:i32) -> Result<(), Box<dyn std::error::Error>>
{
    mouse.move_to(x, y)
}
fn release_mouse(mouse:&Mouse) -> Result<(), Box<dyn std::error::Error>>
{
    mouse.release(&Keys::LEFT)
}
fn mouse_control(mouse:&Mouse)
{
    mouse.move_to(500,500).expect("Unable to move");
    mouse.press(&Keys::RIGHT).expect("Unable to press");
    // mouse.press(&Keys::LEFT).expect("Can't click");
    // mouse.move_to(700,700).expect("Unable to move");
    mouse.release(&Keys::RIGHT).expect("Unable to let go");
    mouse.move_to(0,0).expect("Unable to move");
}
fn move_mouse(mouse:&Mouse, x:i32, y:i32) -> Result<(), Box<(dyn std::error::Error + 'static)>>
{
    mouse.move_to(x, y)
}
fn get_mouse_position(mouse:&Mouse) -> (i32, i32)       
{
    let position:Point = mouse.get_position().unwrap();
    (position.x, position.y)
}
fn execute_command(exe: &str, args: &[&str]) -> Result<Output, std::io::Error>
{
    // let command:Output = Command::new(exe).args(&*args).output().expect("Can't run");
    Command::new(exe).args(&*args).output()
}
fn write_output_to_file(vector_bytes:&Vec<u8>, file_name: &'static str) -> Result<(), std::io::Error>
{
    // Learing about creating a file
    let mut file:File = File::create(file_name)?;
    
    /*
        Learning about iterators
        let mut buffer:[u8; 1024] = [0u8; 1024];
        vector_bytes.iter().map(|item| {
            let number = item;
            let buffer = [..item];
            file.write([item]);
        });
    */ 
    let success:Result<(), std::io::Error> = file.write_all(&vector_bytes);
    
    success
}
fn copy_all_files_in_directory(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()>
{
    fs::create_dir(&destination)?;
    for entry in fs::read_dir(&source)?
    {
        let entry:DirEntry = entry?;
        let try_get_file_type:FileType = entry.file_type()?;
        if try_get_file_type.is_dir()
        {
            copy_all_files_in_directory(entry.path(), &destination.as_ref().join(entry.file_name()))?;
        }else {
            fs::copy(entry.path(), &destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}