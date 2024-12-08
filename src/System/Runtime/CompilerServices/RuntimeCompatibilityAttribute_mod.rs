#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeCompatibilityAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _WrapNonExceptionThrows_k__BackingField: bool,
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::RuntimeCompatibilityAttribute =>
    "System.Runtime.CompilerServices"."RuntimeCompatibilityAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::RuntimeCompatibilityAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::RuntimeCompatibilityAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
impl crate::System::Runtime::CompilerServices::RuntimeCompatibilityAttribute {
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
    pub fn set_WrapNonExceptionThrows(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WrapNonExceptionThrows", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeCompatibilityAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::RuntimeCompatibilityAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
