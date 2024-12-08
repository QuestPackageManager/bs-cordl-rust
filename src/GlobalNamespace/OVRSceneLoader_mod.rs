#[cfg(feature = "OVRSceneLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneLoader {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub sceneCheckIntervalSeconds: f32,
    pub logCloseTime: f32,
    pub mainCanvas: *mut crate::UnityEngine::Canvas,
    pub logTextBox: *mut crate::UnityEngine::UI::Text,
    pub loadSceneOperation: *mut crate::UnityEngine::AsyncOperation,
    pub formattedLogText: *mut crate::System::String,
    pub closeLogTimer: f32,
    pub closeLogDialogue: bool,
    pub canvasPosUpdated: bool,
    pub scenePath: *mut crate::System::String,
    pub sceneLoadDataPath: *mut crate::System::String,
    pub loadedAssetBundles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::AssetBundle,
    >,
    pub currentSceneInfo: crate::GlobalNamespace::OVRSceneLoader_SceneInfo,
}
#[cfg(feature = "OVRSceneLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRSceneLoader => ""."OVRSceneLoader"
);
#[cfg(feature = "OVRSceneLoader")]
impl std::ops::Deref for OVRSceneLoader {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneLoader")]
impl std::ops::DerefMut for OVRSceneLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneLoader")]
impl OVRSceneLoader {
    pub const externalStoragePath: &'static str = "/sdcard/Android/data";
    pub const resourceBundleName: &'static str = "asset_resources";
    pub const sceneLoadDataName: &'static str = "SceneLoadData.txt";
    #[cfg(feature = "OVRSceneLoader+_onCheckSceneCoroutine_d__25")]
    pub type _onCheckSceneCoroutine_d__25 = crate::GlobalNamespace::OVRSceneLoader__onCheckSceneCoroutine_d__25;
    #[cfg(feature = "OVRSceneLoader+SceneInfo")]
    pub type SceneInfo = crate::GlobalNamespace::OVRSceneLoader_SceneInfo;
    #[cfg(feature = "OVRSceneLoader+_DelayCanvasPosUpdate_d__24")]
    pub type _DelayCanvasPosUpdate_d__24 = crate::GlobalNamespace::OVRSceneLoader__DelayCanvasPosUpdate_d__24;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn DelayCanvasPosUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("DelayCanvasPosUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroyAllGameObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllGameObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSceneInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSceneLoader_SceneInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSceneLoader_SceneInfo = __cordl_object
            .invoke("GetSceneInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadScene(
        &mut self,
        sceneInfo: crate::GlobalNamespace::OVRSceneLoader_SceneInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadScene", (sceneInfo))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneOperation_completed(
        &mut self,
        obj: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSceneOperation_completed", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCanvasPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCanvasPosition", ())?;
        Ok(__cordl_ret)
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
    pub fn onCheckSceneCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("onCheckSceneCoroutine", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSceneLoader")]
impl quest_hook::libil2cpp::ObjectType for OVRSceneLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneLoader+SceneInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSceneLoader_SceneInfo {
    pub scenes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub version: i64,
}
#[cfg(feature = "OVRSceneLoader+SceneInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneLoader_SceneInfo => ""
    ."OVRSceneLoader/SceneInfo"
);
#[cfg(feature = "OVRSceneLoader+SceneInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSceneLoader_SceneInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSceneLoader+SceneInfo")]
impl crate::GlobalNamespace::OVRSceneLoader_SceneInfo {
    pub fn _ctor(
        &mut self,
        sceneList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        currentSceneEpochVersion: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sceneList, currentSceneEpochVersion),
        )?;
        Ok(__cordl_ret)
    }
}
