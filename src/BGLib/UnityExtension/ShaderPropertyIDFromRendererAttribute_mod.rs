#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyIDFromRendererAttribute {
    __cordl_parent: crate::BGLib::UnityExtension::ShaderPropertyIDAttribute,
    pub propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute =>
    "BGLib.UnityExtension"."ShaderPropertyIDFromRendererAttribute"
);
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute {
    type Target = crate::BGLib::UnityExtension::ShaderPropertyIDAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
impl crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute {
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
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, nameFilter, filterPropType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, nameFilter, filterPropType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
