use std::{ fs::{self, DirEntry, File, FileType}, io::{self, Error, Read, Write}, num::NonZeroI128, path::Path, process::{ Command, Output }, str, time::Duration };
use mouse_rs::{ types::{keys::Keys, Point}, Mouse };
use windows::Win32::{Foundation::*, Graphics::Gdi::{DFC_BUTTON, BeginPaint, CreatePen, CreateSolidBrush, DeleteObject, DrawFrameControl, DrawTextExA, Ellipse, EndPaint, FillRect, GetStockObject, Rectangle, SelectObject, TextOutW, ValidateRect, DC_PEN, DFCS_BUTTONPUSH, HDC, HGDIOBJ, HPEN, PAINTSTRUCT, PS_SOLID}, System::LibraryLoader::*, UI::{Input::KeyboardAndMouse::{VK_LBUTTON, VK_RBUTTON}, WindowsAndMessaging::*}};
use windows::Win32::Graphics::Direct2D::*;
use windows::core::{ s, HRESULT, PSTR };
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
        let window:HWND = CreateWindowExA(WS_EX_OVERLAPPEDWINDOW | WS_EX_TOPMOST, window_class, s!("Sample window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CAPTION,0,0,X_SIZE,Y_SIZE,None, None, instance,None);
        println!("{:?}",window);
        let mut message = MSG::default();
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
        let instance = GetModuleHandleW(None);
       //  h_result:HRESULT = CreateGraphicsResources();
        match message {
            WM_ACTIVATEAPP => {
                println!("Active app: {:?}", message);
                LRESULT(0)
            },
            WM_PAINT => {
                println!("Paint APP: {:?}", message);
                // PAINTSTRUCT { hdc: val, fErase: val, rcPaint: val, fRestore: val, fIncUpdate: val, rgbReserved: val }
                let mut paint_struct:PAINTSTRUCT = PAINTSTRUCT { ..Default::default() };
                let hdc:HDC = BeginPaint(window, &mut paint_struct);
                // let rect:RECT = RECT {left:0, top:0, right:X_SIZE,bottom:Y_SIZE};
                // println!("RECT: {:?}",&rect);
                // let _ = ValidateRect(window, None);
                
                let mut client_rect: RECT = RECT {..Default::default()};
                let _ = GetClientRect(window,&mut client_rect);
                
                // let mut original_object:HGDIOBJ = HGDIOBJ(0);
                // original_object = SelectObject(paint_struct.hdc, GetStockObject(DC_PEN));
                // let colour_pen:HPEN = CreatePen(PS_SOLID, 5,COLORREF(0x000FFA41));
                // SelectObject(paint_struct.hdc, colour_pen);
                // let _ = Rectangle(paint_struct.hdc, client_rect.left + 100, client_rect.top + 100, client_rect.right - 100, client_rect.bottom - 100);

                // let numbers:Vec<u16> = vec![67, 117, 114, 114, 101, 110, 116, 72, 111, 114, 105, 122, 111, 110, 116, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 67, 117, 114, 114, 101, 110, 116, 86, 101, 114, 116, 105, 99, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 13, 13, 10, 50, 53, 54, 48];
                // // let msg: Vec<u8> = b"Peace!".to_vec();#
                // let _ = Ellipse(hdc, 0, 100, 400, 400);
                // // CreateEllipseGeometry()
                // let mut rect: RECT = RECT {left:0, top:0, right:100,bottom:100};

                // let _ = TextOutW(hdc, 0, 100, &numbers);
                // // DrawTextExA(hdc, &mut msg, &mut rect, DT_LEFT | DT_TOP, None);
                // FillRect(hdc, &mut rect, CreateSolidBrush(COLORREF(0x00A12345)));
                // let delete:BOOL = DeleteObject(colour_pen);
                
                // https://learn.microsoft.com/en-us/windows/win32/direct2d/getting-started-with-direct2d
                // https://learn.microsoft.com/en-us/windows/win32/learnwin32/your-first-direct2d-program?source=recommendations
                //https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct2D/struct.D2D1_RENDER_TARGET_PROPERTIES.html
                //https://learn.microsoft.com/en-us/windows/win32/direct2d/getting-started-with-direct2d
                // let direct_factory:Option<*const D2D1_FACTORY_OPTIONS> = None;
                // let mut direct_factory:Option<*const D2D1_FACTORY_OPTIONS> = Some(std::ptr::null());
                let direct_factory:D2D1_FACTORY_OPTIONS = D2D1_FACTORY_OPTIONS {debugLevel: D2D1_DEBUG_LEVEL(0)};
                let debug_level:*const D2D1_FACTORY_OPTIONS = &direct_factory;
                // let mut factory = std::ptr::null_mut();
                // let h_window_button = CreateMDIWindowA(s!("BUTTON"),s!("button text"), WS_TABSTOP | WS_VISIBLE|WS_CHILD, 10, 10, 100, 100, window, GetWindowLongPtrA(window, GWLP_HINSTANCE),None);
                let h_window_button:HWND = CreateWindowExA(WS_EX_LAYERED,s!("window"),s!("BUTTON"), WS_TABSTOP | WS_VISIBLE|WS_CHILD, 10, 10, 100, 100, window, None,HINSTANCE::default(),None);
                println!("{:?}",h_window_button);
                let button = CREATESTRUCTA{lpCreateParams:std::ptr::null_mut(),hInstance:HINSTANCE(isize::MAX),hwndParent:window,cy:100, cx:100,y:0,x:0,style:BS_PUSHBUTTON | BS_FLAT  ,lpszName:s!("BUTTON"),lpszClass:s!("BUTTON"),..Default::default()};
                // https://learn.microsoft.com/en-us/windows/win32/controls/button-types-and-styles
                // https://stackoverflow.com/questions/11379421/how-to-create-an-embedded-text-input-box-in-win32-windows
                // direct_factory.ok_or(Error::from(E_FAIL))?
                // let handle_result = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(factory));
                // ShowWindow(h_window_button,SW_SHOW);
                DrawFrameControl(hdc, &mut client_rect, DFC_BUTTON, DFCS_BUTTONPUSH);

                let _ = EndPaint(window, &paint_struct);
                LRESULT(0)
            },
            WM_CLOSE => {
                println!("Close APP: {:?}", message);
                PostQuitMessage(0);
                LRESULT(0)
            },
            WM_CREATE => {
                println!("Create App: {:?}", message);
                match instance {
                    Ok(ok)=> {
                        CreateWindowExA(WS_EX_APPWINDOW, s!("BUTTON"), s!("CLICK"),  WS_VISIBLE | WS_CHILD, 10, 10, 100, 100, window,None,ok,None);
                        ShowWindow(window, SW_SHOWDEFAULT);
                    },
                    _ => {
                        CreateWindowExA(WS_EX_APPWINDOW, s!("BUTTON"), s!("CLICK Other"),  WS_VISIBLE | WS_CHILD, 10, 10, 100, 100, window,None,HINSTANCE::default(),None);
                        ShowWindow(window, SW_SHOWDEFAULT);
                    }
                };
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
fn create_graphics_resources() -> HRESULT {
    let h_result: HRESULT  = S_OK;



    h_result

}