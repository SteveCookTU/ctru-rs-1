pub struct Apt(());

impl Apt {
    pub fn init() -> crate::Result<Apt> {
        unsafe {
            let r = ctru_sys::aptInit();
            if r < 0 {
                Err(r.into())
            } else {
                Ok(Apt(()))
            }
        }
    }

    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    pub fn set_app_cpu_time_limit(&self, percent: u32) -> crate::Result<()> {
        unsafe {
            let r = ctru_sys::APT_SetAppCpuTimeLimit(percent);
            if r < 0 {
                Err(r.into())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_chainloader(&self, program_id: u64, media_type: u8) {
        unsafe {
            ctru_sys::aptSetChainloader(program_id, media_type);
        }
    }
}

impl Drop for Apt {
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}
