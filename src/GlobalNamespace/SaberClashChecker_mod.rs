#[cfg(feature = "SaberClashChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberClashChecker {
    __cordl_parent: crate::System::Object,
    pub _sabersAreClashing: bool,
    pub _clashingPoint: crate::UnityEngine::Vector3,
    pub _leftSaber: *mut Saber,
    pub _rightSaber: *mut Saber,
    pub _prevGetFrameNum: i32,
}
#[cfg(feature = "SaberClashChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SaberClashChecker => ""."SaberClashChecker"
);
#[cfg(feature = "SaberClashChecker")]
impl std::ops::Deref for SaberClashChecker {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberClashChecker")]
impl std::ops::DerefMut for SaberClashChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberClashChecker")]
impl SaberClashChecker {
    pub const kIgnoredTime: f32 = 0.1f32;
    pub const kMinDistanceToClash: f32 = 0.08f32;
    pub fn AreSabersClashing(
        &mut self,
        clashingPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AreSabersClashing", (clashingPoint))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        saberManager: *mut SaberManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (saberManager))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SegmentToSegmentDist(
        &mut self,
        fromA: crate::UnityEngine::Vector3,
        toA: crate::UnityEngine::Vector3,
        fromB: crate::UnityEngine::Vector3,
        toB: crate::UnityEngine::Vector3,
        inbetweenPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("SegmentToSegmentDist", (fromA, toA, fromB, toB, inbetweenPoint))?;
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
#[cfg(feature = "SaberClashChecker")]
impl quest_hook::libil2cpp::ObjectType for SaberClashChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
