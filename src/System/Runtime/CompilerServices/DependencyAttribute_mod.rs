#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DependencyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub dependentAssembly: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub loadHint: crate::System::Runtime::CompilerServices::LoadHint,
}
#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::DependencyAttribute =>
    "System.Runtime.CompilerServices"."DependencyAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::DependencyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::DependencyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
impl crate::System::Runtime::CompilerServices::DependencyAttribute {
    pub fn New(
        dependentAssemblyArgument: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        loadHintArgument: crate::System::Runtime::CompilerServices::LoadHint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dependentAssemblyArgument, loadHintArgument))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        dependentAssemblyArgument: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        loadHintArgument: crate::System::Runtime::CompilerServices::LoadHint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dependentAssemblyArgument, loadHintArgument))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DependencyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::DependencyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
