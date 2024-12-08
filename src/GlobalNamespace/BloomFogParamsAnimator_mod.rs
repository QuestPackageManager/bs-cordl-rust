#[cfg(feature = "BloomFogParamsAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogParamsAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bloomFog: *mut BloomFogSO,
}
#[cfg(feature = "BloomFogParamsAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomFogParamsAnimator => ""."BloomFogParamsAnimator"
);
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::Deref for BloomFogParamsAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::DerefMut for BloomFogParamsAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl BloomFogParamsAnimator {
    #[cfg(feature = "BloomFogParamsAnimator+_AnimationCoroutine_d__2")]
    pub type _AnimationCoroutine_d__2 = crate::GlobalNamespace::BloomFogParamsAnimator__AnimationCoroutine_d__2;
    pub fn AnimateBloomFogParamsChange(
        &mut self,
        envFogParams: *mut BloomFogEnvironmentParams,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateBloomFogParamsChange", (envFogParams, duration))?;
        Ok(__cordl_ret)
    }
    pub fn AnimationCoroutine(
        &mut self,
        envFogParams: *mut BloomFogEnvironmentParams,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("AnimationCoroutine", (envFogParams, duration))?;
        Ok(__cordl_ret)
    }
    pub fn GetDefaultBloomFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BloomFogEnvironmentParams> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BloomFogEnvironmentParams = __cordl_object
            .invoke("GetDefaultBloomFogParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetBloomFogParamsChange(
        &mut self,
        envFogParams: *mut BloomFogEnvironmentParams,
        transition: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBloomFogParamsChange", (envFogParams, transition))?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultBloomFogParams(
        &mut self,
        newDefaultBloomFogParams: *mut BloomFogEnvironmentParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultBloomFogParams", (newDefaultBloomFogParams))?;
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
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl quest_hook::libil2cpp::ObjectType for BloomFogParamsAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
