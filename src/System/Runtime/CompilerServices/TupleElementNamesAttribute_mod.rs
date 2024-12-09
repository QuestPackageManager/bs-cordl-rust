#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleElementNamesAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _transformNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::TupleElementNamesAttribute =>
    "System.Runtime.CompilerServices"."TupleElementNamesAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::TupleElementNamesAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::TupleElementNamesAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
impl crate::System::Runtime::CompilerServices::TupleElementNamesAttribute {
    pub fn New(
        transformNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (transformNames))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        transformNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (transformNames))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TupleElementNamesAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::TupleElementNamesAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
