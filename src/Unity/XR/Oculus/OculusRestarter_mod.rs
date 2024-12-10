#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRestarter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub onAfterRestart: *mut crate::System::Action,
    pub onAfterShutdown: *mut crate::System::Action,
    pub onQuit: *mut crate::System::Action,
    pub onAfterCoroutine: *mut crate::System::Action,
    pub m_Coroutine: *mut crate::UnityEngine::Coroutine,
    pub m_pauseAndRestartCoroutine: *mut crate::UnityEngine::Coroutine,
}
#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::OculusRestarter =>
    "Unity.XR.Oculus"."OculusRestarter"
);
#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusRestarter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusRestarter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
impl crate::Unity::XR::Oculus::OculusRestarter {
    #[cfg(feature = "Unity+XR+Oculus+OculusRestarter+_PauseAndRestartCoroutine_d__22")]
    pub type _PauseAndRestartCoroutine_d__22 = crate::Unity::XR::Oculus::OculusRestarter__PauseAndRestartCoroutine_d__22;
    #[cfg(feature = "Unity+XR+Oculus+OculusRestarter+_RestartCoroutine_d__23")]
    pub type _RestartCoroutine_d__23 = crate::Unity::XR::Oculus::OculusRestarter__RestartCoroutine_d__23;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PauseAndRestart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseAndRestart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PauseAndRestartCoroutine(
        &mut self,
        pauseTimeInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("PauseAndRestartCoroutine", (pauseTimeInSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RestartCoroutine(
        &mut self,
        shouldRestart: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("RestartCoroutine", (shouldRestart))?;
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
    pub fn get_isRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isRunning", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusRestarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
