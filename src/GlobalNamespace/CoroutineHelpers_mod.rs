#[cfg(feature = "CoroutineHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CoroutineHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "CoroutineHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CoroutineHelpers => ""
    ."CoroutineHelpers"
);
#[cfg(feature = "CoroutineHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::CoroutineHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::CoroutineHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineHelpers")]
impl crate::GlobalNamespace::CoroutineHelpers {
    pub fn ExecuteAfterDelayCoroutine(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        timeSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteAfterDelayCoroutine", (action, timeSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAfterFrameEnd(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteAfterFrameEnd", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartSingleCoroutine(
        coroutineStarter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICoroutineStarter,
        >,
        handle: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Coroutine>,
        routine: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartSingleCoroutine", (coroutineStarter, handle, routine))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopSingleCoroutine(
        coroutineStarter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICoroutineStarter,
        >,
        handle: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Coroutine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopSingleCoroutine", (coroutineStarter, handle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "CoroutineHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CoroutineHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
