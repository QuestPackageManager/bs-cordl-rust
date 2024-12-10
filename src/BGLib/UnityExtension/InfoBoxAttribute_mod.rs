#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InfoBoxAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub info: *mut quest_hook::libil2cpp::Il2CppString,
    pub messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::InfoBoxAttribute =>
    "BGLib.UnityExtension"."InfoBoxAttribute"
);
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
impl std::ops::Deref for crate::BGLib::UnityExtension::InfoBoxAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::InfoBoxAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
impl crate::BGLib::UnityExtension::InfoBoxAttribute {
    #[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute+Type")]
    pub type Type = crate::BGLib::UnityExtension::InfoBoxAttribute_Type;
    pub fn New(
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, messageType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageType: crate::BGLib::UnityExtension::InfoBoxAttribute_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, messageType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::InfoBoxAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoBoxAttribute_Type {
    Error = 3i32,
    Info = 1i32,
    None = 0i32,
    Warning = 2i32,
}
#[cfg(feature = "BGLib+UnityExtension+InfoBoxAttribute+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::InfoBoxAttribute_Type =>
    "BGLib.UnityExtension"."InfoBoxAttribute/Type"
);
