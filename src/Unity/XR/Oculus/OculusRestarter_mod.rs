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
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::OculusRestarter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusRestarter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PauseAndRestart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "PauseAndRestart", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PauseAndRestartCoroutine(
        &mut self,
        pauseTimeInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("PauseAndRestartCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "PauseAndRestartCoroutine",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (pauseTimeInSeconds))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetCallbacks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "ResetCallbacks", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestartCoroutine(
        &mut self,
        shouldRestart: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("RestartCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "RestartCoroutine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (shouldRestart))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::XR::Oculus::OculusRestarter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Unity::XR::Oculus::OculusRestarter>,
                0usize,
            >("get_Instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "get_Instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::XR::Oculus::OculusRestarter,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PauseAndRestartAttempts() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_PauseAndRestartAttempts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_PauseAndRestartAttempts", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeBetweenRestartAttempts() -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), f32, 0usize>("get_TimeBetweenRestartAttempts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_TimeBetweenRestartAttempts", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isRunning")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(), "get_isRunning", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_TimeBetweenRestartAttempts(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Unity::XR::Oculus::OculusRestarter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_TimeBetweenRestartAttempts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Unity::XR::Oculus::OculusRestarter as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_TimeBetweenRestartAttempts", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
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
