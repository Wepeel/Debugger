type Pid = u32;

// TODO: Return result from WriteProcess

/// `ProcessOperator` allows to read and write from a process
pub trait ProcessOperator {
    /// Writes `data` of `length` to the process at given `address`
    unsafe fn write_process(
        &self,
        address: types::AddressType,
        data: types::ConstAddressType,
        length: usize,
    );
    /// Return new instance of [`ProcessOperator`]
    unsafe fn new(pid: Pid) -> Self;
}

pub unsafe fn default_process_operator(pid: Pid) -> impl ProcessOperator {
    #[cfg(windows)]
    return windows::WindowsProcessOperator::new(pid);
    #[cfg(linux)]
    return linux::LinuxProcecssOperator::default();
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
        ) {
            let mut number_bytes_written: usize = 0;
            memoryapi::WriteProcessMemory(
                self.handle,
                address,
                data,
                length,
                &mut number_bytes_written,
            );
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
    pub struct LinuxProcecssOperator {}

    impl ProcessOperator for LinuxProcecssOperator {
        fn write_process(&self, address: usize) {}
    }
}
