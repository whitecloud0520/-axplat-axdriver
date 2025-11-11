use unified_platform::driver::DeviceType;
use unified_platform::mock;
use unified_platform::platform::AxUnifiedPlatform;

fn reset_world() {
    AxUnifiedPlatform::reset_for_tests();
}

#[test]
fn builtin_drivers_registered_and_initialized() {
    reset_world();

    AxUnifiedPlatform::init_early(0, 0);
    AxUnifiedPlatform::init_later(0, 0);

    let summaries = AxUnifiedPlatform::driver_summaries();
    assert_eq!(summaries.len(), 2);
    assert!(summaries.iter().any(|s| s.device_type == DeviceType::Char));
    assert!(summaries.iter().any(|s| s.device_type == DeviceType::Net));

    assert_eq!(mock::irq::PLATFORM_IRQ.last_irq(), Some(32));

    let output = mock::console::PLATFORM_CONSOLE.snapshot();
    let text = core::str::from_utf8(&output).expect("console utf8");
    assert!(text.contains("[console] ready"));
}

#[test]
fn late_init_is_idempotent() {
    reset_world();

    AxUnifiedPlatform::init_later(0, 0);
    AxUnifiedPlatform::init_later(1, 0);

    let summaries = AxUnifiedPlatform::driver_summaries();
    assert_eq!(summaries.len(), 2);
}