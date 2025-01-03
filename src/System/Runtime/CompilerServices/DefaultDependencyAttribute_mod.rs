#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultDependencyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub loadHint: crate::System::Runtime::CompilerServices::LoadHint,
}
#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::DefaultDependencyAttribute =>
    "System.Runtime.CompilerServices"."DefaultDependencyAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::DefaultDependencyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::DefaultDependencyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
impl crate::System::Runtime::CompilerServices::DefaultDependencyAttribute {
    pub fn New(
        loadHintArgument: crate::System::Runtime::CompilerServices::LoadHint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loadHintArgument))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        loadHintArgument: crate::System::Runtime::CompilerServices::LoadHint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loadHintArgument))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DefaultDependencyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::DefaultDependencyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
