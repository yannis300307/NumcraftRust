macro_rules! configure_app {
    ($app_name: expr, $app_name_size: expr, $icon_path: expr, $icon_size: expr) => {
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_name")]
        pub static EADK_APP_NAME: [u8; $app_name_size] = *$app_name;

        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_api_level")]
        pub static EADK_APP_API_LEVEL: u32 = 0;

        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_icon")]
        pub static EADK_APP_ICON: [u8; $icon_size] = *include_bytes!($icon_path);
    };
}

macro_rules! setup_allocator {
    () => {
        #[allow(unused_imports)]
        #[cfg(target_os = "none")]
        use cortex_m;

        #[cfg(target_os = "none")]
        use crate::eadk::adresses::HEAP_START;
        #[cfg(target_os = "none")]
        use eadk::adresses::heap_size;
        #[cfg(target_os = "none")]
        use embedded_alloc::LlffHeap as Heap;

        #[global_allocator]
        #[cfg(target_os = "none")]
        static HEAP: Heap = Heap::empty();

        #[cfg(target_os = "none")]
        extern crate alloc;
    };
}

macro_rules! init_heap {
    () => {
        #[cfg(target_os = "none")]
        {
            let heap_size: usize = heap_size();
            unsafe { HEAP.init(HEAP_START as usize, heap_size) }
        }
    };
}

/// Conditional import. Only import the crate if the target is the calculator.
macro_rules! calc_use {
    ($crate_name: path) => {
        #[cfg(target_os = "none")]
        use $crate_name;
    };
}

/// Conditional import. Only import the crate if the target is the simulator.
macro_rules! sim_use {
    ($crate_name: path) => {
        #[cfg(not(target_os = "none"))]
        use $crate_name;
    };
}