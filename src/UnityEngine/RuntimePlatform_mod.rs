#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuntimePlatform {
    #[default]
    Android = 11i32,
    BlackBerryPlayer = 22i32,
    CloudRendering = 35i32,
    EmbeddedLinuxArm32 = 40i32,
    EmbeddedLinuxArm64 = 39i32,
    EmbeddedLinuxX64 = 41i32,
    EmbeddedLinuxX86 = 42i32,
    FlashPlayer = 15i32,
    GameCoreScarlett = -1i32,
    GameCoreXboxOne = 37i32,
    GameCoreXboxSeries = 36i32,
    IPhonePlayer = 8i32,
    LinuxEditor = 16i32,
    LinuxPlayer = 13i32,
    LinuxServer = 43i32,
    Lumin = 33i32,
    MetroPlayerARM = 20i32,
    MetroPlayerX64 = 19i32,
    MetroPlayerX86 = 18i32,
    NaCl = 12i32,
    OSXDashboardPlayer = 4i32,
    OSXEditor = 0i32,
    OSXPlayer = 1i32,
    OSXServer = 45i32,
    OSXWebPlayer = 3i32,
    PS3 = 9i32,
    PS4 = 25i32,
    PS5 = 38i32,
    PSM = 26i32,
    PSP2 = 24i32,
    QNXArm32 = 46i32,
    QNXArm64 = 47i32,
    QNXX64 = 48i32,
    QNXX86 = 49i32,
    SamsungTVPlayer = 28i32,
    Stadia = 34i32,
    Switch = 32i32,
    TizenPlayer = 23i32,
    VisionOS = 50i32,
    WP8Player = 21i32,
    WebGLPlayer = 17i32,
    WiiU = 30i32,
    WindowsEditor = 7i32,
    WindowsPlayer = 2i32,
    WindowsServer = 44i32,
    WindowsWebPlayer = 5i32,
    XBOX360 = 10i32,
    XboxOne = 27i32,
    tvOS = 31i32,
}
#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::RuntimePlatform {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "RuntimePlatform";
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
#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::RuntimePlatform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::RuntimePlatform {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::RuntimePlatform {
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
#[cfg(feature = "cordl_class_UnityEngine+RuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::RuntimePlatform {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
