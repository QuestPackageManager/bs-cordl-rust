#[cfg(feature = "ICoroutineStarter")]
#[repr(C)]
#[derive(Debug)]
pub struct ICoroutineStarter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ICoroutineStarter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ICoroutineStarter => ""
    ."ICoroutineStarter"
);
#[cfg(feature = "ICoroutineStarter")]
impl std::ops::Deref for crate::GlobalNamespace::ICoroutineStarter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICoroutineStarter")]
impl std::ops::DerefMut for crate::GlobalNamespace::ICoroutineStarter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICoroutineStarter")]
impl crate::GlobalNamespace::ICoroutineStarter {
    pub fn StartCoroutine(
        &mut self,
        routine: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Coroutine> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Coroutine = __cordl_object
            .invoke("StartCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn StopCoroutine(
        &mut self,
        routine: *mut crate::UnityEngine::Coroutine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCoroutine", (routine))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ICoroutineStarter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ICoroutineStarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
