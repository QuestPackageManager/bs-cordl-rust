#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct IsReadOnlyAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::IsReadOnlyAttribute =>
    "System.Runtime.CompilerServices"."IsReadOnlyAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::IsReadOnlyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::IsReadOnlyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
impl crate::System::Runtime::CompilerServices::IsReadOnlyAttribute {
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
#[cfg(feature = "System+Runtime+CompilerServices+IsReadOnlyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::IsReadOnlyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}