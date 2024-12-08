#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct ErrorWrapper {
    __cordl_parent: crate::System::Object,
    pub m_ErrorCode: i32,
}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::ErrorWrapper
    => "System.Runtime.InteropServices"."ErrorWrapper"
);
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::ErrorWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ErrorWrapper")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::ErrorWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
