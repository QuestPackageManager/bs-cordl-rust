#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BasePoseProvider {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::XR::Interaction::BasePoseProvider =>
    "UnityEngine.Experimental.XR.Interaction"."BasePoseProvider"
);
#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
impl crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider {
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
    pub fn GetPoseFromProvider(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = __cordl_object
            .invoke("GetPoseFromProvider", (output))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPoseFromProvider(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPoseFromProvider", (output))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Experimental+XR+Interaction+BasePoseProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::XR::Interaction::BasePoseProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
