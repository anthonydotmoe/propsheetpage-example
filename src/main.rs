use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::{CreatePropertySheetPageW, PropertySheetW, PROPSHEETHEADERW_V2, PROPSHEETPAGEW, PSH_DEFAULT, PSP_DEFAULT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetDlgItemTextW, SetWindowLongPtrW, DLGPROC, DWLP_MSGRESULT, WINDOW_LONG_PTR_INDEX, WM_COMMAND, WM_INITDIALOG
};

const IDD_PROPSHEET: i32 = 1001;
const IDB_PLUS1: i32 = 1002;
const IDB_MINUS1: i32 = 1003;
const IDC_NUMBER: i32 = 1004;

fn main() {
    let hinstance = HINSTANCE(unsafe {GetModuleHandleW(None)}.unwrap().0);

    // Create the property sheet page (this is what I would like winsafe to do)
    let mut psp = PROPSHEETPAGEW::default();
    psp.dwSize = std::mem::size_of::<PROPSHEETPAGEW>() as u32;
    psp.dwFlags = PSP_DEFAULT;
    psp.hInstance = hinstance;
    psp.Anonymous1.pszTemplate = PCWSTR(IDD_PROPSHEET as *const u16);
    psp.pfnDlgProc = Some(dlgproc);

    let hpsp = unsafe {
        CreatePropertySheetPageW(&mut psp)
    };

    // MMC would do the next part

    // Collect all HPROPSHEETPAGE from snap-ins
    let mut pages = vec![hpsp];

    // Create the property sheet header
    let mut psh = PROPSHEETHEADERW_V2::default();
    psh.dwSize = std::mem::size_of::<PROPSHEETHEADERW_V2>() as u32;
    psh.dwFlags = PSH_DEFAULT;
    psh.nPages = pages.len() as u32;
    psh.Anonymous2.nStartPage = 0;
    psh.Anonymous3.phpage = pages.as_mut_ptr();

    // Create the property sheet
    let _ = unsafe {PropertySheetW(&mut psh)};

}

const DWLP_DLGPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(DWLP_MSGRESULT as i32 + std::mem::size_of::<LRESULT>() as i32);
const DWLP_USER: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(DWLP_DLGPROC.0 + std::mem::size_of::<DLGPROC>() as i32);

unsafe fn update_counter(hwnd: HWND, d: isize) {
    let text: Vec<u16> = format!("{}", d)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let _ = SetDlgItemTextW(hwnd, IDC_NUMBER, PCWSTR(text.as_ptr()));

    SetWindowLongPtrW(hwnd, DWLP_USER, d);
}

pub unsafe extern "system" fn dlgproc(hwnd: HWND, msg: u32, wparam: WPARAM, _lparam: LPARAM) -> isize {
    let state = match msg {
        WM_INITDIALOG => {
            // Initialize the state on WM_INITDIALOG
            let state = 0;
            SetWindowLongPtrW(hwnd, DWLP_USER, state);
            state
        },
        _ => GetWindowLongPtrW(hwnd, DWLP_USER),
    };

    match msg {
        WM_COMMAND => {
            match wparam.0 as i32 {
                IDB_PLUS1 => {
                    update_counter(hwnd, state + 1);
                },
                IDB_MINUS1 => {
                    update_counter(hwnd, state - 1);
                },
                _ => {}
            }
            return 1;
        },
        _ => {},
    }

    0
}