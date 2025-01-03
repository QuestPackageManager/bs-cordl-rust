#[cfg(feature = "CreditsController")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _creditsSceneSetupDataSO: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CreditsScenesTransitionSetupDataSO,
    >,
    pub _audioPlayer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioPlayerBase>,
    pub _continueButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _contentRectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub _overflowHeight: f32,
    pub _contentWrapper: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _beforeMusicPacksCredits: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    >,
    pub _afterMusicPacksCredits: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _packDefinitions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::PackDefinitionSO,
        >,
    >,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _didFinish: bool,
    pub _isPaused: bool,
}
#[cfg(feature = "CreditsController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsController => ""
    ."CreditsController"
);
#[cfg(feature = "CreditsController")]
impl std::ops::Deref for crate::GlobalNamespace::CreditsController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsController")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreditsController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsController")]
impl crate::GlobalNamespace::CreditsController {
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHmdMounted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHmdMounted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHmdUnmounted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHmdUnmounted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusReleased", ())?;
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
    pub fn PauseCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseCredits", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("ScrollCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnAllCredits(
        &mut self,
        musicPackCredits: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnAllCredits", (musicPackCredits))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnCredits(
        &mut self,
        creditsToSpawn: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::GameObject,
            >,
        >,
        initialHeight: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("SpawnCredits", (creditsToSpawn, initialHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("StartAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnpauseCredits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnpauseCredits", ())?;
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
}
#[cfg(feature = "CreditsController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CreditsController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
