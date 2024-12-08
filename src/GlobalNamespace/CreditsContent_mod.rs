#[cfg(feature = "CreditsContent")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsContent {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _normalTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _normalLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _titleTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _titleLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _headerTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _headerLocalizedTextPrefab: *mut crate::UnityEngine::GameObject,
    pub _columnCount: i32,
    pub _spaceHeight: f32,
    pub _titleHeight: f32,
    pub _contentRoot: *mut crate::UnityEngine::Transform,
    pub _rootRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _creditsContentTextAsset: *mut crate::UnityEngine::TextAsset,
}
#[cfg(feature = "CreditsContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsContent => ""
    ."CreditsContent"
);
#[cfg(feature = "CreditsContent")]
impl std::ops::Deref for crate::GlobalNamespace::CreditsContent {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsContent")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreditsContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsContent")]
impl crate::GlobalNamespace::CreditsContent {
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
    pub fn get_columnCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_columnCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_contentRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_creditsContentTextAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_creditsContentTextAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headerLocalizedTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_headerLocalizedTextPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headerTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_headerTextPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalLocalizedTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_normalLocalizedTextPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normalTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_normalTextPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootRectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_rootRectTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spaceHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spaceHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_titleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_titleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_titleLocalizedTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_titleLocalizedTextPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_titleTextPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_titleTextPrefab", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CreditsContent")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CreditsContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
