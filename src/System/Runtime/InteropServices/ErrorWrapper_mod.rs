#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct ErrorWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ErrorCode: i32,
}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::ErrorWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "ErrorWrapper";
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
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::ErrorWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::ErrorWrapper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl crate::System::Runtime::InteropServices::ErrorWrapper {}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ErrorWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
