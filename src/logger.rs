pub fn start() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .without_timestamps()
        .with_module_level("wgpu_core", log::LevelFilter::Info)
        .with_module_level("wgpu_core::device", log::LevelFilter::Warn)
        .with_module_level("naga::valid::function", log::LevelFilter::Info)
        .with_module_level("naga::front", log::LevelFilter::Info)
        .with_module_level("wgpu_hal::auxil", log::LevelFilter::Error)
        .with_module_level("wgpu_hal::vulkan::instance", log::LevelFilter::Warn)
        .init().unwrap();
}