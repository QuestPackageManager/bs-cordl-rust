#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum GraphicsDeviceType {
    #[cfg_attr(feature = "derive_Default", default)]
    Direct3D11 = 2i32,
    Direct3D12 = 18i32,
    Direct3D9 = 1i32,
    GameCoreScarlett = -1i32,
    GameCoreXboxOne = 24i32,
    GameCoreXboxSeries = 25i32,
    Metal = 16i32,
    N3DS = 19i32,
    Null = 4i32,
    OpenGL2 = 0i32,
    OpenGLCore = 17i32,
    OpenGLES2 = 8i32,
    OpenGLES3 = 11i32,
    PlayStation3 = 3i32,
    PlayStation4 = 13i32,
    PlayStation5 = 26i32,
    PlayStation5NGGC = 27i32,
    PlayStationMobile = 15i32,
    PlayStationVita = 12i32,
    ReservedCFE = 29i32,
    Switch = 22i32,
    Vulkan = 21i32,
    WebGPU = 28i32,
    Xbox360 = 6i32,
    XboxOne = 14i32,
    XboxOneD3D12 = 23i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::GraphicsDeviceType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GraphicsDeviceType";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::GraphicsDeviceType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::GraphicsDeviceType {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::GraphicsDeviceType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GraphicsDeviceType")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::GraphicsDeviceType {
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
