#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum GraphicsFormatUsage {
    #[default]
    Blend = 32i32,
    GetPixels = 64i32,
    Linear = 2i32,
    LoadStore = 1024i32,
    MSAA2x = 2048i32,
    MSAA4x = 4096i32,
    MSAA8x = 8192i32,
    None = 0i32,
    ReadPixels = 512i32,
    Render = 16i32,
    Sample = 1i32,
    SetPixels = 128i32,
    SetPixels32 = 256i32,
    Sparse = 4i32,
    StencilSampling = 65536i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUsage
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "GraphicsFormatUsage";
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUsage
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUsage
{
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUsage
{
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUsage")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUsage
{
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
