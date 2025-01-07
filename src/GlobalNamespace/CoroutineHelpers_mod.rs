#[cfg(feature = "CoroutineHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CoroutineHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "CoroutineHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CoroutineHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CoroutineHelpers";
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
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
        >,
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
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
        >,
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
