#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::EventBuilder =>
    "System.Reflection.Emit"."EventBuilder"
);
#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::EventBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::EventBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
impl crate::System::Reflection::Emit::EventBuilder {}
#[cfg(feature = "System+Reflection+Emit+EventBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::EventBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
