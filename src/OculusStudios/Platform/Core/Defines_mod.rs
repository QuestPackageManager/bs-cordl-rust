#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+Defines")]
#[repr(C)]
#[derive(Debug)]
pub struct Defines {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+Defines")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::Platform::Core::Defines {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Core";
    const CLASS_NAME: &'static str = "Defines";
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
#[cfg(feature = "OculusStudios+Platform+Core+Defines")]
impl std::ops::Deref for crate::OculusStudios::Platform::Core::Defines {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+Defines")]
impl std::ops::DerefMut for crate::OculusStudios::Platform::Core::Defines {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+Defines")]
impl crate::OculusStudios::Platform::Core::Defines {
    pub const TIMEOUT_WEBREQUEST_SECONDS: u32 = 20u32;
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+Defines")]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::Platform::Core::Defines {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
