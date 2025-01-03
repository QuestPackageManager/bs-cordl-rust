#[cfg(feature = "LocalizedTextAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _textInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
        >,
    >,
    pub _lastTextInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
    >,
}
#[cfg(feature = "LocalizedTextAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalizedTextAsset => ""
    ."LocalizedTextAsset"
);
#[cfg(feature = "LocalizedTextAsset")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedTextAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedTextAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl crate::GlobalNamespace::LocalizedTextAsset {
    #[cfg(feature = "LocalizedTextAsset+TextInfo")]
    pub type TextInfo = crate::GlobalNamespace::LocalizedTextAsset_TextInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localizedText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::LocalizedTextAsset_TextInfo,
            >,
        > = __cordl_object.invoke("get_textInfos", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalizedTextAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalizedTextAsset {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub language: crate::BGLib::Polyglot::Language,
    pub localizedText: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
}
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalizedTextAsset_TextInfo =>
    ""."LocalizedTextAsset/TextInfo"
);
#[cfg(feature = "LocalizedTextAsset+TextInfo")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedTextAsset_TextInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
