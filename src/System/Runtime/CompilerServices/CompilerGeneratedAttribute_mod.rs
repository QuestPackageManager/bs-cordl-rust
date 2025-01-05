#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CompilerGeneratedAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::CompilerGeneratedAttribute =>
    "System.Runtime.CompilerServices"."CompilerGeneratedAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::CompilerGeneratedAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::CompilerGeneratedAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
impl crate::System::Runtime::CompilerServices::CompilerGeneratedAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilerGeneratedAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CompilerGeneratedAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
