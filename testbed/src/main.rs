use engine::core;

fn main() {
    core::LogFatal!("A test message: {}", std::f64::consts::PI);
    core::LogError!("A test message: {}", std::f64::consts::PI);
    core::LogWarn!("A test message: {}", std::f64::consts::PI);
    core::LogInfo!("A test message: {}", std::f64::consts::PI);
    core::LogDebug!("A test message: {}", std::f64::consts::PI);
    core::LogTrace!("A test message: {}", std::f64::consts::PI);

    core::Assert!(1 == 0);
}
