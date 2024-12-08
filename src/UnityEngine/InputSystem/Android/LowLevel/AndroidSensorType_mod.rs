#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidSensorType {
    Accelerometer = 1i32,
    AccelerometerUncalibrated = 35i32,
    AmbientTemperature = 13i32,
    GameRotationVector = 15i32,
    GeomagneticRotationVector = 20i32,
    Gravity = 9i32,
    Gyroscope = 4i32,
    GyroscopeUncalibrated = 16i32,
    HeartBeat = 31i32,
    HeartRate = 21i32,
    Light = 5i32,
    LinearAcceleration = 10i32,
    LowLatencyOffBodyDetect = 34i32,
    MagneticField = 2i32,
    MagneticFieldUncalibrated = 14i32,
    MotionDetect = 30i32,
    None = 0i32,
    Orientation = 3i32,
    Pose6DOF = 28i32,
    Pressure = 6i32,
    Proximity = 8i32,
    RelativeHumidity = 12i32,
    RotationVector = 11i32,
    SignificantMotion = 17i32,
    StationaryDetect = 29i32,
    StepCounter = 19i32,
    StepDetector = 18i32,
    Temperature = 7i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidSensorType"
);
