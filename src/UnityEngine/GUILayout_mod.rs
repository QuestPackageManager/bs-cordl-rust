#[cfg(feature = "UnityEngine+GUILayout")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayout {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+GUILayout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayout => "UnityEngine"
    ."GUILayout"
);
#[cfg(feature = "UnityEngine+GUILayout")]
impl std::ops::Deref for crate::UnityEngine::GUILayout {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayout")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayout")]
impl crate::UnityEngine::GUILayout {
    pub fn BeginArea_Gc_Gc1(
        screenRect: crate::UnityEngine::Rect,
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginArea", (screenRect, content, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginArea_Rect0(
        screenRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginArea", (screenRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginHorizontal_Gc0(
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginHorizontal", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginHorizontal_Gc_Gc1(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginHorizontal", (content, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginVertical_Gc0(
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginVertical", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginVertical_Gc_Gc1(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginVertical", (content, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoLabel(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoLabel", (content, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndArea() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndArea", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndVertical() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndVertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Height(
        height: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutOption,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Height", (height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Label(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Label", (text, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Space(
        pixels: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Space", (pixels))?;
        Ok(__cordl_ret.into())
    }
    pub fn Width(
        width: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutOption,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Width", (width))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayout")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
