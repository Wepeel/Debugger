type Pid = u32;

use std::error::Error;

#[derive(Debug)]
pub struct ProcessOperatorError {}

impl Error for ProcessOperatorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}

impl std::fmt::Display for ProcessOperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return "error with process operations".fmt(f);
    }
}

type Result<T> = std::result::Result<T, ProcessOperatorError>;

// TODO: Return result from WriteProcess

/// `ProcessOperator` allows to read and write from a process
pub trait ProcessOperator {
    /// Writes `data` of `length` to the process at given `address`
    unsafe fn write_process(
        &self,
        address: types::AddressType,
        data: types::ConstAddressType,
        length: usize,
    ) -> self::Result<()>;
    unsafe fn read_process(
        &self,
        address: types::ConstAddressType,
        data: types::AddressType,
        length: usize,
    ) -> self::Result<()>;
    /// Return new instance of [`ProcessOperator`]
    unsafe fn new(pid: Pid) -> Self;
}

pub unsafe fn default_process_operator(pid: Pid) -> impl ProcessOperator {
    #[cfg(windows)]
    return windows::WindowsProcessOperator::new(pid);
    #[cfg(linux)]
    return linux::LinuxProcessOperator::default();
}

#[cfg(windows)]
pub mod types {
    use winapi::shared::minwindef;
    pub type AddressType = minwindef::LPVOID;
    pub type ConstAddressType = minwindef::LPCVOID;
}

#[cfg(windows)]
pub mod windows {
    use winapi::shared::minwindef;
    use winapi::um::errhandlingapi;
    use winapi::um::memoryapi;
    use winapi::um::processthreadsapi;
    use winapi::um::winnt;

    use super::types;

    const PROCESS_VM_READ: minwindef::DWORD = 0x0010;
    const PROCESS_VM_WRITE: minwindef::DWORD = 0x0020;

    pub struct WindowsProcessOperator {
        handle: winnt::HANDLE,
    }

    impl super::ProcessOperator for WindowsProcessOperator {
        unsafe fn write_process(
            &self,
            address: types::AddressType,
            data: types::ConstAddressType,
            length: usize,
        ) -> super::Result<()> {
            let mut number_bytes_written: usize = 0;
            let success = memoryapi::WriteProcessMemory(
                self.handle,
                address,
                data,
                length,
                &mut number_bytes_written,
            );

            if success == minwindef::FALSE {
                let err_code = errhandlingapi::GetLastError();
                //return Err(err_code);
            }

            return Ok(());
        }
        unsafe fn read_process(
            &self,
            address: types::ConstAddressType,
            data: types::AddressType,
            length: usize,
        ) -> super::Result<()> {
            let mut number_bytes_read: usize = 0;
            let success = memoryapi::ReadProcessMemory(
                self.handle,
                address,
                data,
                length,
                &mut number_bytes_read,
            );

            if success == minwindef::FALSE {
                let err_code = errhandlingapi::GetLastError();
                //return Err(err_code);
            }

            return Ok(());
        }
        unsafe fn new(pid: super::Pid) -> Self {
            let handle = processthreadsapi::OpenProcess(
                PROCESS_VM_READ | PROCESS_VM_WRITE,
                minwindef::FALSE,
                pid,
            );
            return WindowsProcessOperator { handle };
        }
    }
}

#[cfg(linux)]
pub mod linux {
    pub struct LinuxProcessOperator {}

    impl ProcessOperator for LinuxProcessOperator {
        fn write_process(&self, address: usize) {}
    }
}
