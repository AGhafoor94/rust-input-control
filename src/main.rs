use std::{ fs::{self, DirEntry, File, FileType}, io::{self, Read, Write}, path::Path, process::{ Command, Output }, str, time::Duration };
use mouse_rs::{ types::{keys::Keys, Point}, Mouse };
use windows::Win32::{Foundation::*, Graphics::{Direct2D::*, Gdi::{BeginPaint, CreatePen, CreateSolidBrush, DeleteObject, DrawTextExA, Ellipse, EndPaint, FillRect, GetStockObject, MapWindowPoints, Rectangle, SelectObject, TextOutW, ValidateRect, DC_PEN, HDC, HGDIOBJ, HPEN, PAINTSTRUCT, PS_SOLID}}, System::LibraryLoader::*, UI::{Input::KeyboardAndMouse::{VK_LBUTTON, VK_RBUTTON}, WindowsAndMessaging::*}};
use windows::core::{ s, PWSTR };
// use std::io::{ Error };
static X_SIZE: i32 = 500;
static Y_SIZE:i32 = 500;
fn main() -> Result<(), std::io::Error>
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
    // let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    // mouse_control(&mouse);
    // let (position_x, position_y):(i32,i32) = get_mouse_position(&mouse);
    // println!("{:?}", (position_x, position_y));
    // // execute_command("cmd", &["/C","echo hello"]);
    // // execute_command("cmd", &["/C","start msedge"]);
    // // execute_command("cmd", &["/C","explorer https://www.office.com"]);
    // let _ = press_hold_mouse(&mouse);
    // let _ = move_mouse_to_location(&mouse, 1280, 720);
    // // let _ = move_mouse_to_location(&mouse, 40, 20);
    // std::thread::sleep(Duration::from_secs(5));
    // let _ = release_mouse(&mouse);
    unsafe {
        // let title:PWSTR = [0b1010];
        // let caption:HSTRING = "World".into();
        
        // let message_box_result:MESSAGEBOX_RESULT = MessageBoxA(None, s!("Caption"), s!("Title"), MB_YESNO | MB_ICONASTERISK | MB_TOPMOST |MB_SETFOREGROUND );
        // println!("{:?}",message_box_result.0);
        // match message_box_result.0
        // {
        //     6 => println!("Message set to true"),
        //     7 => println!("Message set to false"),
        //     _ => println!("Error")
        // };
        // let hwnd:HWND = HWND(0);
        // println!("{:?}",hwnd.0);
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);
        let window_class = s!("window");
        let window_class_a = WNDCLASSA {
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: window_class,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: Default::default(),
            hbrBackground: Default::default(),
            lpszMenuName: s!("Menu")
        };
        let atom = RegisterClassA(&window_class_a);
        debug_assert!(atom != 0);
        println!("Atom: {:?}",atom);
        // dwexstyle below: WS_EX_OVERLAPPEDWINDOW | WS_EX_TOPMOST
        let window:HWND = CreateWindowExA(Default::default(), window_class, s!("Sample window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_SYSMENU | WS_MINIMIZEBOX,0,0,X_SIZE,Y_SIZE,HWND_DESKTOP, None, instance,None);
        println!("{:?}",window);
        let mut message = MSG::default();
        let current_window:HWND = GetForegroundWindow();
        let lenght = GetWindowTextLengthA(current_window);
        let mut window_text: Vec<u16> = vec![];
        GetWindowTextW(current_window,&mut window_text);
        let mut client_rect: RECT = RECT {..Default::default()};
        let _ = GetClientRect(current_window,&mut client_rect);
        SetWindowPos(window,HWND_TOP,1280,720,client_rect.right,client_rect.bottom,SWP_SHOWWINDOW);
        // let windows_position = MapWindowPoints(0, current_window,0 );
        let mut text:Vec<u16> = vec![0;lenght as usize];
        let get_text_from_window = GetWindowTextW(current_window,&mut text);
        println!("line 80: {:?}",String::from_utf16_lossy(&text[..lenght as usize]));
        // GetMessageA(&mut message, None, 0,0);
        while GetMessageA(&mut message, None, 0, 0).into(){
            DispatchMessageA(&message);
        }
    }
    Ok(())
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
extern "system" fn wnd_proc(window:HWND, message:u32, wparam:WPARAM, lparam:LPARAM) -> LRESULT
{
    // println!("Message: {:?}", message);
    unsafe {
       //  h_result:HRESULT = CreateGraphicsResources();
       
        match message {
            WM_ACTIVATEAPP => {
                println!("Active app: {:?}", message);
                LRESULT(0)
            },
            WM_CREATE => {
                println!("Create App: {:?}", message);
                
                let _ = CreateWindowExA(Default::default(),s!("STATIC"), s!("Test message"),WS_VISIBLE | WS_CHILD, 20,20, 300, 20, window,HMENU(1),None,None);
                let _ = CreateWindowExA(Default::default(), s!("BUTTON"), s!("Text inside"), WS_VISIBLE | WS_CHILD| WS_BORDER, 20, 50, 200, 20, window, HMENU(2), None, None);
                let _ = CreateWindowExA(Default::default(), s!("BUTTON"), s!("Close"), WS_VISIBLE | WS_CHILD| WS_BORDER, 20, 150, 200, 20, window, HMENU(3), None, None);
                let _ = CreateWindowExA(Default::default(), s!("EDIT"), s!("Hello World"),WS_VISIBLE | WS_CHILD| WS_BORDER, 30, 180, 200, 50, window, None,None, None);
                LRESULT(0)
            },
            WM_COMMAND => {
                // let code_val = wparam.0 as i32;
                match wparam.0 as i32 {
                    1 => {
                        MessageBoxExA(window, s!("Button Pressed"), s!("BUTTON PRESSED"), MB_OK, Default::default());
                    },
                    2 => {
                        println!("2");
                    },
                    3 => {
                        PostQuitMessage(0);
                    },
                    4 => {
                        let mut text:Vec<u8>= vec![];
                        let val:i32 = GetWindowTextA(window, &mut text);
                        println!("199: {:?}", val);
                    },
                    _ => {
                        println!("Error");
                        let mut buffer: Vec<u16> = vec![0; (100 + 1) as usize];
                        let mut text:Vec<u8>= vec![];
                        let val:i32 = GetWindowTextW(window, &mut buffer);
                        let window_text = String::from_utf16_lossy(&buffer[..100 as usize]);

                        println!("Active window text: {}", window_text);
                    }
                }
                println!("189: {:?}", wparam);

                LRESULT(0)
            },
            // WM_PAINT => {
            //     println!("Paint APP: {:?}", message);
            //     // PAINTSTRUCT { hdc: val, fErase: val, rcPaint: val, fRestore: val, fIncUpdate: val, rgbReserved: val }
            //     let mut paint_struct:PAINTSTRUCT = PAINTSTRUCT { ..Default::default() };
            //     let hdc:HDC = BeginPaint(window, &mut paint_struct);
            //     // let rect:RECT = RECT {left:0, top:0, right:X_SIZE,bottom:Y_SIZE};
            //     // println!("RECT: {:?}",&rect);
            //     // let _ = ValidateRect(window, None);
                
            //     let mut client_rect: RECT = RECT {..Default::default()};
            //     let _ = GetClientRect(window,&mut client_rect);
            //     let mut original_object:HGDIOBJ = HGDIOBJ(0);
            //     original_object = SelectObject(paint_struct.hdc, GetStockObject(DC_PEN));
            //     let black_pen:HPEN = CreatePen(PS_SOLID, 3,COLORREF(0x000FFAA1));
            //     SelectObject(paint_struct.hdc, black_pen);
            //     let _ = Rectangle(paint_struct.hdc, client_rect.left + 100, client_rect.top + 100, client_rect.right - 100, client_rect.bottom - 100);

            //     let numbers:Vec<u16> = vec![67, 117, 114, 114, 101, 110, 116, 72, 111, 114, 105, 122, 111, 110, 116, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 67, 117, 114, 114, 101, 110, 116, 86, 101, 114, 116, 105, 99, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 13, 13, 10, 50, 53, 54, 48];
            //     // let msg: Vec<u8> = b"Peace!".to_vec();#
            //     let _ = Ellipse(hdc, 0, 100, 400, 400);
            //     // CreateEllipseGeometry()
            //     let mut rect: RECT = RECT {left:0, top:0, right:100,bottom:100};

            //     let _ = TextOutW(hdc, 0, 100, &numbers);
            //     // DrawTextExA(hdc, &mut msg, &mut rect, DT_LEFT | DT_TOP, None);
            //     FillRect(hdc, &mut rect, CreateSolidBrush(COLORREF(0x00A12345)));
            //     let delete:BOOL = DeleteObject(black_pen);
            //     let _ = EndPaint(window, &paint_struct);
            //     LRESULT(0)
            // },
            WM_CLOSE => {
                println!("Close APP: {:?}", message);
                PostQuitMessage(0);
                LRESULT(0)
            },
            WM_DESTROY => {
                println!("Destoryed: {:?}", message);
                PostQuitMessage(0);
                LRESULT(0)
            },
            _ => DefWindowProcA(window, message, wparam, lparam)
        }
    }
}