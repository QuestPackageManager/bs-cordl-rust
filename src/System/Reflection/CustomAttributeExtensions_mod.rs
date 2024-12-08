#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomAttributeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::CustomAttributeExtensions =>
    "System.Reflection"."CustomAttributeExtensions"
);
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl std::ops::Deref for crate::System::Reflection::CustomAttributeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl std::ops::DerefMut for crate::System::Reflection::CustomAttributeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl crate::System::Reflection::CustomAttributeExtensions {}
#[cfg(feature = "System+Reflection+CustomAttributeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::CustomAttributeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
