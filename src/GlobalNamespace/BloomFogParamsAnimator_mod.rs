#[cfg(feature = "BloomFogParamsAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogParamsAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bloomFog: *mut crate::GlobalNamespace::BloomFogSO,
}
#[cfg(feature = "BloomFogParamsAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomFogParamsAnimator => ""
    ."BloomFogParamsAnimator"
);
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFogParamsAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFogParamsAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl crate::GlobalNamespace::BloomFogParamsAnimator {
    #[cfg(feature = "BloomFogParamsAnimator+_AnimationCoroutine_d__2")]
    pub type _AnimationCoroutine_d__2 = crate::GlobalNamespace::BloomFogParamsAnimator__AnimationCoroutine_d__2;
    pub fn AnimateBloomFogParamsChange(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateBloomFogParamsChange", (envFogParams, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn AnimationCoroutine(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("AnimationCoroutine", (envFogParams, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultBloomFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogEnvironmentParams>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        > = __cordl_object.invoke("GetDefaultBloomFogParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetBloomFogParamsChange(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        transition: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBloomFogParamsChange", (envFogParams, transition))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultBloomFogParams(
        &mut self,
        newDefaultBloomFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultBloomFogParams", (newDefaultBloomFogParams))?;
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
#[cfg(feature = "BloomFogParamsAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomFogParamsAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
