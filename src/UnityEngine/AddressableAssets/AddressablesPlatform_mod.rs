#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressablesPlatform {
    #[default]
    Android = 9i32,
    Linux = 3i32,
    OSX = 2i32,
    PS4 = 4i32,
    Switch = 5i32,
    Unknown = 0i32,
    WebGL = 7i32,
    Windows = 1i32,
    WindowsUniversal = 10i32,
    XboxOne = 6i32,
    iOS = 8i32,
}
#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::AddressablesPlatform {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets";
    const CLASS_NAME: &'static str = "AddressablesPlatform";
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
#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::AddressableAssets::AddressablesPlatform {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::AddressableAssets::AddressablesPlatform {
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
#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::AddressableAssets::AddressablesPlatform {
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
#[cfg(feature = "cordl_class_UnityEngine+AddressableAssets+AddressablesPlatform")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::AddressableAssets::AddressablesPlatform {
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
