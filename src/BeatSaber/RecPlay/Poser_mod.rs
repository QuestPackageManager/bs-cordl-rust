#[cfg(feature = "BeatSaber+RecPlay+Poser")]
#[repr(C)]
#[derive(Debug)]
pub struct Poser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::RecPlay::Poser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.RecPlay";
    const CLASS_NAME: &'static str = "Poser";
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
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::Poser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::Poser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl crate::BeatSaber::RecPlay::Poser {
    pub fn InterpolatePose(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolatePose", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvertPose(
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvertPose", (pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn MirrorPoseYZ(
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MirrorPoseYZ", (pose))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+Poser")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::Poser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
