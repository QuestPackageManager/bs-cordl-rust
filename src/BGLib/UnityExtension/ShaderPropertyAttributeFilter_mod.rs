#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderPropertyAttributeFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    pub nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::ShaderPropertyAttributeFilter => "BGLib.UnityExtension"
    ."ShaderPropertyAttributeFilter"
);
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl std::ops::Deref for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    #[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
    pub type PropType = crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType;
    pub fn New(
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameFilter, propType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nameFilter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propType: crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameFilter, propType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::ShaderPropertyAttributeFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderPropertyAttributeFilter_PropType {
    #[default]
    Any = 0i32,
    Color = 1i32,
    Float = 3i32,
    Int = 6i32,
    Range = 4i32,
    Texture = 5i32,
    Vector = 2i32,
}
#[cfg(feature = "BGLib+UnityExtension+ShaderPropertyAttributeFilter+PropType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::ShaderPropertyAttributeFilter_PropType =>
    "BGLib.UnityExtension"."ShaderPropertyAttributeFilter/PropType"
);
