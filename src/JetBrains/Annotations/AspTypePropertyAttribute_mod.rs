#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AspTypePropertyAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _CreateConstructorReferences_k__BackingField: bool,
}
#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::AspTypePropertyAttribute
    => "JetBrains.Annotations"."AspTypePropertyAttribute"
);
#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::AspTypePropertyAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::AspTypePropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
impl crate::JetBrains::Annotations::AspTypePropertyAttribute {
    pub fn New(
        createConstructorReferences: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (createConstructorReferences))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        createConstructorReferences: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (createConstructorReferences))?;
        Ok(__cordl_ret)
    }
    pub fn get_CreateConstructorReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CreateConstructorReferences", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CreateConstructorReferences(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CreateConstructorReferences", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "JetBrains+Annotations+AspTypePropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AspTypePropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
