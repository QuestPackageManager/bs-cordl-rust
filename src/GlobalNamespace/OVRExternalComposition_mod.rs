#[cfg(feature = "OVRExternalComposition")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRExternalComposition {
    __cordl_parent: crate::GlobalNamespace::OVRComposition,
    pub previousMainCameraObject: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub foregroundCameraGameObject: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub foregroundCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub backgroundCameraGameObject: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub backgroundCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub skipFrame: bool,
    pub fpsThreshold: f32,
    pub isFrameSkipped: bool,
    pub renderCombinedFrame: bool,
    pub audioListener: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioListener>,
    pub audioFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRMRAudioFilter>,
    pub mrcRenderTextureArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::RenderTexture>,
    >,
    pub frameIndex: i32,
    pub lastMrcEncodeFrameSyncId: i32,
    pub mrcForegroundRenderTextureArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::RenderTexture>,
    >,
    pub cameraPoseTimeArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f64>,
    >,
    pub cachedAudioDataArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub audioDataLock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cachedAudioData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<f32>,
    >,
    pub cachedChannels: i32,
}
#[cfg(feature = "OVRExternalComposition")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRExternalComposition => ""
    ."OVRExternalComposition"
);
#[cfg(feature = "OVRExternalComposition")]
impl std::ops::Deref for crate::GlobalNamespace::OVRExternalComposition {
    type Target = crate::GlobalNamespace::OVRComposition;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExternalComposition")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRExternalComposition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRExternalComposition")]
impl crate::GlobalNamespace::OVRExternalComposition {
    pub fn CacheAudioData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        channels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CacheAudioData", (data, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn CastMrcFrame(
        &mut self,
        castTextureIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CastMrcFrame", (castTextureIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanupAudioFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupAudioFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = __cordl_object
            .invoke("CompositionMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisplayRefreshRateChanged(
        &mut self,
        fromRefreshRate: f32,
        toRefreshRate: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayRefreshRateChanged", (fromRefreshRate, toRefreshRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAndResetAudioData(
        &mut self,
        audioData: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        audioFrames: quest_hook::libil2cpp::ByRefMut<i32>,
        channels: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAndResetAudioData", (audioData, audioFrames, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshAudioFilter(
        &mut self,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAudioFilter", (mainCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshCameraObjects(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshCameraObjects", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraTargetTexture(
        &mut self,
        drawTextureIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCameraTargetTexture", (drawTextureIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (gameObject, mainCamera, configuration, trackingOrigin))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRExternalComposition")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRExternalComposition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
