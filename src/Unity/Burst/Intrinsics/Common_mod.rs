#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
#[repr(C)]
#[derive(Debug)]
pub struct Common {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Common =>
    "Unity.Burst.Intrinsics"."Common"
);
#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::Common {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::Common {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
impl crate::Unity::Burst::Intrinsics::Common {
    pub fn Pause() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn umul128(
        x: u64,
        y: u64,
        high: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("umul128", (x, y, high))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Common")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::Common {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
