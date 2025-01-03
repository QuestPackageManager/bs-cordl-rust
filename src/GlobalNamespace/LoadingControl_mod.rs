#[cfg(feature = "LoadingControl")]
#[repr(C)]
#[derive(Debug)]
pub struct LoadingControl {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _loadingContainer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _loadingText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _refreshText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _refreshButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _refreshContainer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _downloadingContainer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _downloadingText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _donwloadingProgressImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub didPressRefreshButtonEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
}
#[cfg(feature = "LoadingControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LoadingControl => ""
    ."LoadingControl"
);
#[cfg(feature = "LoadingControl")]
impl std::ops::Deref for crate::GlobalNamespace::LoadingControl {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LoadingControl")]
impl std::ops::DerefMut for crate::GlobalNamespace::LoadingControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LoadingControl")]
impl crate::GlobalNamespace::LoadingControl {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowDownloadingProgress(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        downloadingProgress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowDownloadingProgress", (text, downloadingProgress))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowLoading(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLoading", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        showRefreshButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowText", (text, showRefreshButton))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__14_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__14_0", ())?;
        Ok(__cordl_ret.into())
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
    pub fn add_didPressRefreshButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressRefreshButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isLoading(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isLoading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressRefreshButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressRefreshButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LoadingControl")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LoadingControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
