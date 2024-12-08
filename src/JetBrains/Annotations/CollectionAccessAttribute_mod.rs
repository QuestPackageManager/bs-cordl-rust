#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionAccessAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _CollectionAccessType_k__BackingField: crate::JetBrains::Annotations::CollectionAccessType,
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::JetBrains::Annotations::CollectionAccessAttribute => "JetBrains.Annotations"
    ."CollectionAccessAttribute"
);
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::CollectionAccessAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::CollectionAccessAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl crate::JetBrains::Annotations::CollectionAccessAttribute {
    pub fn New(
        collectionAccessType: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collectionAccessType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        collectionAccessType: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collectionAccessType))?;
        Ok(__cordl_ret)
    }
    pub fn get_CollectionAccessType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::JetBrains::Annotations::CollectionAccessType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::JetBrains::Annotations::CollectionAccessType = __cordl_object
            .invoke("get_CollectionAccessType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CollectionAccessType(
        &mut self,
        value: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CollectionAccessType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::CollectionAccessAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
