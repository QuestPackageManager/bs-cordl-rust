#[cfg(feature = "UnityEngine+Cloth")]
#[repr(C)]
#[derive(Debug)]
pub struct Cloth {
    __cordl_parent: crate::UnityEngine::Component,
    pub _useContinuousCollision_k__BackingField: f32,
    pub _selfCollision_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+Cloth")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Cloth => "UnityEngine"."Cloth"
);
#[cfg(feature = "UnityEngine+Cloth")]
impl std::ops::Deref for crate::UnityEngine::Cloth {
    type Target = crate::UnityEngine::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cloth")]
impl std::ops::DerefMut for crate::UnityEngine::Cloth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cloth")]
impl crate::UnityEngine::Cloth {
    pub fn set_externalAcceleration(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_externalAcceleration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_externalAcceleration_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_externalAcceleration_Injected", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Cloth")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Cloth {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
