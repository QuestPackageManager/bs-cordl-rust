#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AndroidAxis {
    #[default]
    Brake = 23i32,
    Distance = 24i32,
    Gas = 22i32,
    Generic1 = 32i32,
    Generic10 = 41i32,
    Generic11 = 42i32,
    Generic12 = 43i32,
    Generic13 = 44i32,
    Generic14 = 45i32,
    Generic15 = 46i32,
    Generic16 = 47i32,
    Generic2 = 33i32,
    Generic3 = 34i32,
    Generic4 = 35i32,
    Generic5 = 36i32,
    Generic6 = 37i32,
    Generic7 = 38i32,
    Generic8 = 39i32,
    Generic9 = 40i32,
    HatX = 15i32,
    HatY = 16i32,
    Hscroll = 10i32,
    Ltrigger = 17i32,
    Orientation = 8i32,
    Pressure = 2i32,
    Rtrigger = 18i32,
    Rudder = 20i32,
    Rx = 12i32,
    Ry = 13i32,
    Rz = 14i32,
    Size = 3i32,
    Throttle = 19i32,
    Tilt = 25i32,
    ToolMajor = 6i32,
    ToolMinor = 7i32,
    TouchMajor = 4i32,
    TouchMinor = 5i32,
    Vscroll = 9i32,
    Wheel = 21i32,
    X = 0i32,
    Y = 1i32,
    Z = 11i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Android.LowLevel";
    const CLASS_NAME: &'static str = "AndroidAxis";
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
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis {
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
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis {
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
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidAxis")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis {
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
