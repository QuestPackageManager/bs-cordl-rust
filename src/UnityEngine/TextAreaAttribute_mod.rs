#[cfg(feature = "UnityEngine+TextAreaAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TextAreaAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub minLines: i32,
    pub maxLines: i32,
}
#[cfg(feature = "UnityEngine+TextAreaAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextAreaAttribute => "UnityEngine"
    ."TextAreaAttribute"
);
#[cfg(feature = "UnityEngine+TextAreaAttribute")]
impl std::ops::Deref for crate::UnityEngine::TextAreaAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextAreaAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::TextAreaAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextAreaAttribute")]
impl crate::UnityEngine::TextAreaAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_1(
        minLines: i32,
        maxLines: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minLines, maxLines))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        minLines: i32,
        maxLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minLines, maxLines))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextAreaAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextAreaAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}