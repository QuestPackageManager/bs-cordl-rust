#[cfg(feature = "UnityEngine+FrameTimingManager")]
#[repr(C)]
#[derive(Debug)]
pub struct FrameTimingManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FrameTimingManager => "UnityEngine"
    ."FrameTimingManager"
);
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl std::ops::Deref for crate::UnityEngine::FrameTimingManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl std::ops::DerefMut for crate::UnityEngine::FrameTimingManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl crate::UnityEngine::FrameTimingManager {
    pub fn CaptureFrameTimings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CaptureFrameTimings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLatestTimings(
        numFrames: u32,
        timings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::FrameTiming>,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLatestTimings", (numFrames, timings))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::FrameTimingManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
