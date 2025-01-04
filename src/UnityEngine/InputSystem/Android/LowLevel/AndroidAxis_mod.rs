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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidAxis"
);
