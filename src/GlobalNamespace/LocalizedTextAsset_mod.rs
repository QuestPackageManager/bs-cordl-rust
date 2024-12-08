#[cfg(feature = "LocalizedTextAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _textInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
    >,
    pub _lastTextInfo: *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
}
#[cfg(feature = "LocalizedTextAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LocalizedTextAsset => ""."LocalizedTextAsset"
);
#[cfg(feature = "LocalizedTextAsset")]
impl std::ops::Deref for LocalizedTextAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl std::ops::DerefMut for LocalizedTextAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl LocalizedTextAsset {
    #[cfg(feature = "LocalizedTextAsset+__c")]
    pub type __c = crate::GlobalNamespace::LocalizedTextAsset___c;
    #[cfg(feature = "LocalizedTextAsset+TextInfo")]
    pub type TextInfo = crate::GlobalNamespace::LocalizedTextAsset_TextInfo;
    #[cfg(feature = "LocalizedTextAsset+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::LocalizedTextAsset___c__DisplayClass6_0;
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
    pub fn get_localizedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
        > = __cordl_object.invoke("get_textInfos", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl quest_hook::libil2cpp::ObjectType for LocalizedTextAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextAsset_TextInfo {
    __cordl_parent: crate::System::Object,
    pub language: crate::BGLib::Polyglot::Language,
    pub localizedText: *mut crate::UnityEngine::TextAsset,
}
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalizedTextAsset_TextInfo =>
    ""."LocalizedTextAsset/TextInfo"
);
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedTextAsset_TextInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedTextAsset_TextInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
impl crate::GlobalNamespace::LocalizedTextAsset_TextInfo {
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
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalizedTextAsset_TextInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
