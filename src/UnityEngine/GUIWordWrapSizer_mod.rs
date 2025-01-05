#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIWordWrapSizer {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutEntry>,
    pub m_Content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
    pub m_ForcedMinHeight: f32,
    pub m_ForcedMaxHeight: f32,
}
#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIWordWrapSizer => "UnityEngine"
    ."GUIWordWrapSizer"
);
#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
impl std::ops::Deref for crate::UnityEngine::GUIWordWrapSizer {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutEntry>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
impl std::ops::DerefMut for crate::UnityEngine::GUIWordWrapSizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
impl crate::UnityEngine::GUIWordWrapSizer {
    pub fn CalcHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalcWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (style, content, options))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (style, content, options))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUIWordWrapSizer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIWordWrapSizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
