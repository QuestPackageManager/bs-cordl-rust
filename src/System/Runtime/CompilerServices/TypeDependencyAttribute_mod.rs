#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeDependencyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::TypeDependencyAttribute =>
    "System.Runtime.CompilerServices"."TypeDependencyAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::TypeDependencyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::TypeDependencyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
impl crate::System::Runtime::CompilerServices::TypeDependencyAttribute {
    pub fn New(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeDependencyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::TypeDependencyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
