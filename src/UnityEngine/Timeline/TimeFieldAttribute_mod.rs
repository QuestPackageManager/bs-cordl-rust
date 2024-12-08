#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeFieldAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub _useEditMode_k__BackingField: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TimeFieldAttribute =>
    "UnityEngine.Timeline"."TimeFieldAttribute"
);
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimeFieldAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimeFieldAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl crate::UnityEngine::Timeline::TimeFieldAttribute {
    #[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
    pub type UseEditMode = crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode;
    pub fn get_useEditMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode = __cordl_object
            .invoke("get_useEditMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        useEditMode: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useEditMode))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        useEditMode: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useEditMode))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimeFieldAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFieldAttribute_UseEditMode {
    ApplyEditMode = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode => "UnityEngine.Timeline"
    ."TimeFieldAttribute/UseEditMode"
);
