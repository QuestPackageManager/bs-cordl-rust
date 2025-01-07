#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AndroidSensorType {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Android.LowLevel";
    const CLASS_NAME: &'static str = "AndroidSensorType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidSensorType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidSensorType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
