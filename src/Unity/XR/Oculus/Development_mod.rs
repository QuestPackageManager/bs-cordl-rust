#[cfg(feature = "Unity+XR+Oculus+Development")]
#[repr(C)]
#[derive(Debug)]
pub struct Development {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Development {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Development";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Development {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Development {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl crate::Unity::XR::Oculus::Development {
    #[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
    pub type UserDeveloperModeSettingCache = crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache;
    pub fn OverrideDeveloperModeStart() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverrideDeveloperModeStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OverrideDeveloperModeStop() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverrideDeveloperModeStop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetDeveloperMode(
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrySetDeveloperMode", (active))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Development {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Development_UserDeveloperModeSettingCache {
    #[default]
    NoUserSettingCached = 0i32,
    UserSettingFalse = 1i32,
    UserSettingTrue = 2i32,
}
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Development/UserDeveloperModeSettingCache";
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
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache {
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
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache {
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
#[cfg(feature = "Unity+XR+Oculus+Development+UserDeveloperModeSettingCache")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::XR::Oculus::Development_UserDeveloperModeSettingCache {
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
