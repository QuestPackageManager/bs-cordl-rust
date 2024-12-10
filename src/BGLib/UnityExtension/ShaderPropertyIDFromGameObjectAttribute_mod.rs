#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyIDFromGameObjectAttribute {
    __cordl_parent: crate::BGLib::UnityExtension::ShaderPropertyIDAttribute,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::ShaderPropertyIDFromGameObjectAttribute =>
    "BGLib.UnityExtension"."ShaderPropertyIDFromGameObjectAttribute"
);
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::ShaderPropertyIDFromGameObjectAttribute {
    type Target = crate::BGLib::UnityExtension::ShaderPropertyIDAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::ShaderPropertyIDFromGameObjectAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
impl crate::BGLib::UnityExtension::ShaderPropertyIDFromGameObjectAttribute {
    pub const kTargetName: &'static str = "GameObject";
    pub fn GetTargetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTargetName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameFilter, filterPropType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameFilter, filterPropType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromGameObjectAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ShaderPropertyIDFromGameObjectAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
