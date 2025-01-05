#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeConstantAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::CustomConstantAttribute,
    >,
    pub _date: crate::System::DateTime,
}
#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::DateTimeConstantAttribute =>
    "System.Runtime.CompilerServices"."DateTimeConstantAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::DateTimeConstantAttribute {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::CustomConstantAttribute,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::DateTimeConstantAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
impl crate::System::Runtime::CompilerServices::DateTimeConstantAttribute {
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+DateTimeConstantAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::DateTimeConstantAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
