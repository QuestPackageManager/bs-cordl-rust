#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SignatureHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::SignatureHelper =>
    "System.Reflection.Emit"."SignatureHelper"
);
#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
impl std::ops::Deref for crate::System::Reflection::Emit::SignatureHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::SignatureHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
impl crate::System::Reflection::Emit::SignatureHelper {}
#[cfg(feature = "System+Reflection+Emit+SignatureHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::SignatureHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
