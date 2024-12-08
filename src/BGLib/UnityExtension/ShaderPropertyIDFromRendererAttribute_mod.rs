#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyIDFromRendererAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyIDFromRendererAttribute {
    __cordl_parent: crate::BGLib::UnityExtension::ShaderPropertyIDAttribute,
    pub propertyName: *mut crate::System::String,
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTargetName", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        propertyName: *mut crate::System::String,
        nameFilter: *mut crate::System::String,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, nameFilter, filterPropType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        propertyName: *mut crate::System::String,
        nameFilter: *mut crate::System::String,
        filterPropType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, nameFilter, filterPropType))?;
        Ok(__cordl_object)
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
