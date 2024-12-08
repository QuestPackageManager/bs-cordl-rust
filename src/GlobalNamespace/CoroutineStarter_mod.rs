#[cfg(feature = "CoroutineStarter")]
#[repr(C)]
#[derive(Debug)]
pub struct CoroutineStarter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "CoroutineStarter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CoroutineStarter => ""."CoroutineStarter"
);
#[cfg(feature = "CoroutineStarter")]
impl std::ops::Deref for CoroutineStarter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineStarter")]
impl std::ops::DerefMut for CoroutineStarter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineStarter")]
impl CoroutineStarter {
    pub fn ICoroutineStarter_StartCoroutine(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("ICoroutineStarter.StartCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn ICoroutineStarter_StopCoroutine(
        &mut self,
        routine: *mut crate::UnityEngine::Coroutine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ICoroutineStarter.StopCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "CoroutineStarter")]
impl quest_hook::libil2cpp::ObjectType for CoroutineStarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}