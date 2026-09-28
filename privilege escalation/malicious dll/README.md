# Description
DLL used for DLL search order hijacking POC written in Rust. It's targeting the KBDUS.dll and placed within the appropriate Edge folder. When Edge executes, the dll is loaded and a connection back to my Sliver C2 is made. This does need local admin privilege in order to work.
# Creating Your Own
It's easy to create and setup a DLL to do what you want it to.
## Tools
* VS Code
* Sliver C2
* Dumpbin.exe
* Procmon.exe

With procmon, you're looking for processes that are trying to load dll's that don't exist within the current path. So the filter should look something like:
* Process is <Target Process>
* Path ends in .dll
* Result is name not found

When you have a dll picked out, you need to see what functions it exports and implement those. You can find the function using dumpbin.
* Dumpbin.exe /exports <Path to the dll>

The code just needs to export the function name for each function found using dumpbin. That is what pub extern "system" is for. The rust compiler also mangles the function name so the #[no_mangle] is also needed for this to work.
# How to Improve
Since the code of the exported function in the bad dll is no longer properly implemented, the victim program is going to crash. Obviously, this is bad opsec and a better way to do it is to forward the export calls.
As this is a simple POC meant for learning and I'm lazy, I'm not gonna do that.

VirtualAlloc is typically monitored so replacing that with the native api, NtAllocateVirtualMemory, would bypass more EDR's. 
