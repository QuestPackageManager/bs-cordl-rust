#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBeatmapEventDataProcessor {
    __cordl_parent: crate::GlobalNamespace::FxBeatmapEventDataProcessor_1<
        *mut crate::GlobalNamespace::FloatFxBeatmapEventData,
    >,
}
#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FloatFxBeatmapEventDataProcessor => ""
    ."FloatFxBeatmapEventDataProcessor"
);
#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxBeatmapEventDataProcessor {
    type Target = crate::GlobalNamespace::FxBeatmapEventDataProcessor_1<
        *mut crate::GlobalNamespace::FloatFxBeatmapEventData,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxBeatmapEventDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
impl crate::GlobalNamespace::FloatFxBeatmapEventDataProcessor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateByOther(
        &mut self,
        current: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBeatmapEventData,
        >,
        other: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateByOther", (current, other))?;
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
#[cfg(feature = "FloatFxBeatmapEventDataProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatFxBeatmapEventDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
