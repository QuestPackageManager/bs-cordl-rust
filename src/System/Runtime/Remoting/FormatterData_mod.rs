#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatterData {
    __cordl_parent: crate::System::Runtime::Remoting::ProviderData,
}
#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::FormatterData =>
    "System.Runtime.Remoting"."FormatterData"
);
#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
impl std::ops::Deref for crate::System::Runtime::Remoting::FormatterData {
    type Target = crate::System::Runtime::Remoting::ProviderData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::FormatterData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
impl crate::System::Runtime::Remoting::FormatterData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+FormatterData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::FormatterData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
