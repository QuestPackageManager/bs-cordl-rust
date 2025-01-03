#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
#[repr(C)]
#[derive(Debug)]
pub struct FxEventBox {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBox,
    pub l: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    pub s: f32,
    pub t: crate::BeatmapSaveDataCommon::DistributionParamType,
    pub i: crate::BeatmapSaveDataCommon::EaseType,
    pub b: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::FxEventBox =>
    "BeatmapSaveDataVersion3"."FxEventBox"
);
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::FxEventBox {
    type Target = crate::BeatmapSaveDataVersion3::EventBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::FxEventBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
impl crate::BeatmapSaveDataVersion3::FxEventBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        >,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        vfxDistributionParam: f32,
        vfxDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        vfxDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        vfxDistributionShouldAffectFirstBaseEvent: bool,
        effectsBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    vfxDistributionParam,
                    vfxDistributionParamType,
                    vfxDistributionEaseType,
                    vfxDistributionShouldAffectFirstBaseEvent,
                    effectsBaseDataList,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        >,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        vfxDistributionParam: f32,
        vfxDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
        vfxDistributionEaseType: crate::BeatmapSaveDataCommon::EaseType,
        vfxDistributionShouldAffectFirstBaseEvent: bool,
        effectsBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    vfxDistributionParam,
                    vfxDistributionParamType,
                    vfxDistributionEaseType,
                    vfxDistributionShouldAffectFirstBaseEvent,
                    effectsBaseDataList,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vfxBaseDataList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<i32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<i32>,
        > = __cordl_object.invoke("get_vfxBaseDataList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vfxDistributionEaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::EaseType = __cordl_object
            .invoke("get_vfxDistributionEaseType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vfxDistributionParam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_vfxDistributionParam", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vfxDistributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::DistributionParamType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::DistributionParamType = __cordl_object
            .invoke("get_vfxDistributionParamType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vfxDistributionShouldAffectFirstBaseEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_vfxDistributionShouldAffectFirstBaseEvent", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBox")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::FxEventBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
