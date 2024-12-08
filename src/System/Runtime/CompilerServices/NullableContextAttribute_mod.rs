#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NullableContextAttribute {
    __cordl_parent: crate::System::Attribute,
    pub Flag: u8,
}
#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::NullableContextAttribute =>
    "System.Runtime.CompilerServices"."NullableContextAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::NullableContextAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::NullableContextAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
impl crate::System::Runtime::CompilerServices::NullableContextAttribute {
    pub fn _ctor(
        &mut self,
        _cordl_fixed_empty_name_whitespace: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_fixed_empty_name_whitespace))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_fixed_empty_name_whitespace: u8,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_fixed_empty_name_whitespace))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+NullableContextAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::NullableContextAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
