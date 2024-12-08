#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TextInfo {
    __cordl_parent: crate::System::Object,
    pub characterCount: i32,
    pub spriteCount: i32,
    pub spaceCount: i32,
    pub wordCount: i32,
    pub linkCount: i32,
    pub lineCount: i32,
    pub pageCount: i32,
    pub materialCount: i32,
    pub textElementInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::TextElementInfo,
    >,
    pub wordInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::WordInfo,
    >,
    pub linkInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::LinkInfo,
    >,
    pub lineInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::LineInfo,
    >,
    pub pageInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::PageInfo,
    >,
    pub meshInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::TextCore::Text::MeshInfo,
    >,
    pub isDirty: bool,
    pub hasMultipleColors: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextInfo =>
    "UnityEngine.TextCore.Text"."TextInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
impl crate::UnityEngine::TextCore::Text::TextInfo {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearLineInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLineInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearMeshInfo(
        &mut self,
        updateMesh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMeshInfo", (updateMesh))?;
        Ok(__cordl_ret)
    }
    pub fn ClearPageInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPageInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::Text::TextInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
