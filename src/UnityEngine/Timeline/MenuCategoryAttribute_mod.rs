#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuCategoryAttribute {
    __cordl_parent: crate::System::Attribute,
    pub category: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::MenuCategoryAttribute =>
    "UnityEngine.Timeline"."MenuCategoryAttribute"
);
#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::MenuCategoryAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::MenuCategoryAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
impl crate::UnityEngine::Timeline::MenuCategoryAttribute {
    pub fn New(
        category: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (category))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        category: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (category))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Timeline+MenuCategoryAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::MenuCategoryAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
