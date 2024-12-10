#[cfg(feature = "IMockBeatmapDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IMockBeatmapDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMockBeatmapDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IMockBeatmapDataProvider => ""
    ."IMockBeatmapDataProvider"
);
#[cfg(feature = "IMockBeatmapDataProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IMockBeatmapDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMockBeatmapDataProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IMockBeatmapDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMockBeatmapDataProvider")]
impl crate::GlobalNamespace::IMockBeatmapDataProvider {
    pub fn GetBeatmapData(
        &mut self,
        beatmap: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MockBeatmapData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MockBeatmapData,
            >,
        > = __cordl_object.invoke("GetBeatmapData", (beatmap, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMockBeatmapDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IMockBeatmapDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IMockBeatmapDataProvider")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::IMockBeatmapDataProvider {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IMockBeatmapDataProvider")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::IMockBeatmapDataProvider {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
