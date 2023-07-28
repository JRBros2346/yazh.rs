use engine::core::logger;
use engine::core::asserts;

fn main() {
    logger::LogFatal!("A test message: {}", std::f64::consts::PI);
    logger::LogError!("A test message: {}", std::f64::consts::PI);
    logger::LogWarn!("A test message: {}", std::f64::consts::PI);
    logger::LogInfo!("A test message: {}", std::f64::consts::PI);
    logger::LogDebug!("A test message: {}", std::f64::consts::PI);
    logger::LogTrace!("A test message: {}", std::f64::consts::PI);

    asserts::Assert!(1 == 0);
}
