#[cfg(feature = "NoteCutInfo+FailReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteCutInfo_FailReason {
    CutHarder = 3i32,
    None = 0i32,
    TooSoon = 1i32,
    WrongColor = 2i32,
    WrongDirection = 4i32,
}
#[cfg(feature = "NoteCutInfo+FailReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutInfo_FailReason => ""
    ."NoteCutInfo/FailReason"
);
#[cfg(feature = "NoteCutInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NoteCutInfo {
    pub noteData: *mut crate::GlobalNamespace::NoteData,
    pub speedOK: bool,
    pub directionOK: bool,
    pub saberTypeOK: bool,
    pub wasCutTooSoon: bool,
    pub saberSpeed: f32,
    pub saberDir: crate::UnityEngine::Vector3,
    pub saberType: crate::GlobalNamespace::SaberType,
    pub timeDeviation: f32,
    pub cutDirDeviation: f32,
    pub cutPoint: crate::UnityEngine::Vector3,
    pub cutNormal: crate::UnityEngine::Vector3,
    pub cutAngle: f32,
    pub cutDistanceToCenter: f32,
    pub worldRotation: crate::UnityEngine::Quaternion,
    pub inverseWorldRotation: crate::UnityEngine::Quaternion,
    pub noteRotation: crate::UnityEngine::Quaternion,
    pub notePosition: crate::UnityEngine::Vector3,
    pub saberMovementData: *mut crate::GlobalNamespace::ISaberMovementData,
}
#[cfg(feature = "NoteCutInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutInfo => ""."NoteCutInfo"
);
#[cfg(feature = "NoteCutInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::NoteCutInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "NoteCutInfo")]
impl crate::GlobalNamespace::NoteCutInfo {
    #[cfg(feature = "NoteCutInfo+FailReason")]
    pub type FailReason = crate::GlobalNamespace::NoteCutInfo_FailReason;
    pub fn _ctor(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
        speedOK: bool,
        directionOK: bool,
        saberTypeOK: bool,
        wasCutTooSoon: bool,
        saberSpeed: f32,
        saberDir: crate::UnityEngine::Vector3,
        saberType: crate::GlobalNamespace::SaberType,
        timeDeviation: f32,
        cutDirDeviation: f32,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        cutDistanceToCenter: f32,
        cutAngle: f32,
        worldRotation: crate::UnityEngine::Quaternion,
        inverseWorldRotation: crate::UnityEngine::Quaternion,
        noteRotation: crate::UnityEngine::Quaternion,
        notePosition: crate::UnityEngine::Vector3,
        saberMovementData: *mut crate::GlobalNamespace::ISaberMovementData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                noteData,
                speedOK,
                directionOK,
                saberTypeOK,
                wasCutTooSoon,
                saberSpeed,
                saberDir,
                saberType,
                timeDeviation,
                cutDirDeviation,
                cutPoint,
                cutNormal,
                cutDistanceToCenter,
                cutAngle,
                worldRotation,
                inverseWorldRotation,
                noteRotation,
                notePosition,
                saberMovementData,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_allExceptSaberTypeIsOK(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_allExceptSaberTypeIsOK",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_allIsOK(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_allIsOK",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_failReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutInfo_FailReason> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutInfo_FailReason = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_failReason",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
