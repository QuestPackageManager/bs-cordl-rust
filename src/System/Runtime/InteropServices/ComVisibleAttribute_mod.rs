#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ComVisibleAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub _val: bool,
}
#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ComVisibleAttribute =>
    "System.Runtime.InteropServices"."ComVisibleAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::ComVisibleAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::ComVisibleAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
impl crate::System::Runtime::InteropServices::ComVisibleAttribute {
    pub fn New(
        visibility: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (visibility))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        visibility: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (visibility))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComVisibleAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ComVisibleAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
