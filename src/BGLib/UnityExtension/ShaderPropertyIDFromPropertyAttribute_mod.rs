#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromPropertyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyIDFromPropertyAttribute {
    __cordl_parent: crate::BGLib::UnityExtension::ShaderPropertyIDFromRendererAttribute,
    pub nestedPropertyName: *mut crate::System::String,
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTargetName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        propertyName: *mut crate::System::String,
        nestedPropertyName: *mut crate::System::String,
        nameFilter: *mut crate::System::String,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (propertyName, nestedPropertyName, nameFilter, filterPropType),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        propertyName: *mut crate::System::String,
        nestedPropertyName: *mut crate::System::String,
        nameFilter: *mut crate::System::String,
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
        Ok(__cordl_ret)
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
