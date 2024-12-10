#[cfg(feature = "UnityEngine+GUILayoutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutUtility => "UnityEngine"
    ."GUILayoutUtility"
);
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl crate::UnityEngine::GUILayoutUtility {
    #[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
    pub type LayoutCache = crate::UnityEngine::GUILayoutUtility_LayoutCache;
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayoutUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutUtility_LayoutCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id_k__BackingField: i32,
    pub topLevel: *mut crate::UnityEngine::GUILayoutGroup,
    pub layoutGroups: *mut crate::UnityEngineInternal::GenericStack,
    pub windows: *mut crate::UnityEngine::GUILayoutGroup,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutUtility_LayoutCache =>
    "UnityEngine"."GUILayoutUtility/LayoutCache"
);
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl crate::UnityEngine::GUILayoutUtility_LayoutCache {
    pub fn New(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (instanceID))?;
        Ok(__cordl_object.into())
    }
    pub fn ResetCursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_id", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
