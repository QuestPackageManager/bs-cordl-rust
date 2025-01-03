#[cfg(feature = "ObstacleSpawnData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ObstacleSpawnData {
    pub moveOffset: crate::UnityEngine::Vector3,
    pub obstacleWidth: f32,
    pub obstacleHeight: f32,
}
#[cfg(feature = "ObstacleSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleSpawnData => ""
    ."ObstacleSpawnData"
);
#[cfg(feature = "ObstacleSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ObstacleSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ObstacleSpawnData")]
impl crate::GlobalNamespace::ObstacleSpawnData {
    pub fn _ctor(
        &mut self,
        moveOffset: crate::UnityEngine::Vector3,
        obstacleWidth: f32,
        obstacleHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (moveOffset, obstacleWidth, obstacleHeight),
        )?;
        Ok(__cordl_ret.into())
    }
}
