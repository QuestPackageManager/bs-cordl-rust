#[cfg(feature = "System+CLSCompliantAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CLSCompliantAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _compliant: bool,
}
#[cfg(feature = "System+CLSCompliantAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::CLSCompliantAttribute => "System"
    ."CLSCompliantAttribute"
);
#[cfg(feature = "System+CLSCompliantAttribute")]
impl std::ops::Deref for crate::System::CLSCompliantAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+CLSCompliantAttribute")]
impl std::ops::DerefMut for crate::System::CLSCompliantAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+CLSCompliantAttribute")]
impl crate::System::CLSCompliantAttribute {
    pub fn New(
        isCompliant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isCompliant))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        isCompliant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isCompliant))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+CLSCompliantAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::System::CLSCompliantAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
