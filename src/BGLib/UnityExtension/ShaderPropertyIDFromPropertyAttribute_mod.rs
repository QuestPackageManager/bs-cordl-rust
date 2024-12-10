#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyIDFromPropertyAttribute {
    __cordl_parent: crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute,
    pub nestedPropertyName: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::ShaderPropertyIDFromPropertyAttribute =>
    "BGLib.UnityExtension"."ShaderPropertyIDFromPropertyAttribute"
);
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
impl std::ops::Deref
for crate::BGLib::UnityExtension::ShaderPropertyIDFromPropertyAttribute {
    type Target = crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
impl std::ops::DerefMut
for crate::BGLib::UnityExtension::ShaderPropertyIDFromPropertyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
impl crate::BGLib::UnityExtension::ShaderPropertyIDFromPropertyAttribute {
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
        nestedPropertyName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (propertyName, nestedPropertyName, nameFilter, filterPropType),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nestedPropertyName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (propertyName, nestedPropertyName, nameFilter, filterPropType),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ShaderPropertyIDFromPropertyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
