#[cfg(feature = "Unity+XR+Oculus+OculusRestarter")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRestarter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub onAfterRestart: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onAfterShutdown: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onQuit: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onAfterCoroutine: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_Coroutine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    pub m_pauseAndRestartCoroutine: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Coroutine,
    >,
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
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::XR::Oculus::OculusRestarter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::XR::Oculus::OculusRestarter,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PauseAndRestartAttempts() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PauseAndRestartAttempts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeBetweenRestartAttempts() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TimeBetweenRestartAttempts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isRunning", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TimeBetweenRestartAttempts(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_TimeBetweenRestartAttempts", (value))?;
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
