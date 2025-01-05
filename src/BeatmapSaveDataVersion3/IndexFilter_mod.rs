#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub f: crate::BeatmapSaveDataCommon::IndexFilterType,
    pub p: i32,
    pub t: i32,
    pub r: i32,
    pub c: i32,
    pub n: crate::BeatmapSaveDataCommon::IndexFilterRandomType,
    pub s: i32,
    pub l: f32,
    pub d: crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType,
}
#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::IndexFilter =>
    "BeatmapSaveDataVersion3"."IndexFilter"
);
#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::IndexFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::IndexFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
impl crate::BeatmapSaveDataVersion3::IndexFilter {
    pub fn CreateDivisionIndexFilter(
        numberOfSections: i32,
        divisionIdx: i32,
        reversed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDivisionIndexFilter",
                (numberOfSections, divisionIdx, reversed),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateForExtension() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateForExtension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStepFilter(
        offset: i32,
        step: i32,
        reversed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::IndexFilter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateStepFilter", (offset, step, reversed))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IndexFilter1(
        other: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IndexFilterType_i32_i32__cordl_bool_IndexFilterRandomType_i32_i32_f32_IndexFilterLimitAlsoAffectsType0(
        _cordl_type: crate::BeatmapSaveDataCommon::IndexFilterType,
        param0: i32,
        param1: i32,
        reversed: bool,
        random: crate::BeatmapSaveDataCommon::IndexFilterRandomType,
        seed: i32,
        chunks: i32,
        limit: f32,
        limitAlsoAffectsType: crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_type,
                    param0,
                    param1,
                    reversed,
                    random,
                    seed,
                    chunks,
                    limit,
                    limitAlsoAffectsType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_IndexFilter1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IndexFilterType_i32_i32__cordl_bool_IndexFilterRandomType_i32_i32_f32_IndexFilterLimitAlsoAffectsType0(
        &mut self,
        _cordl_type: crate::BeatmapSaveDataCommon::IndexFilterType,
        param0: i32,
        param1: i32,
        reversed: bool,
        random: crate::BeatmapSaveDataCommon::IndexFilterRandomType,
        seed: i32,
        chunks: i32,
        limit: f32,
        limitAlsoAffectsType: crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_type,
                    param0,
                    param1,
                    reversed,
                    random,
                    seed,
                    chunks,
                    limit,
                    limitAlsoAffectsType,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_chunks(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_chunks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_limit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limitAlsoAffectsType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType = __cordl_object
            .invoke("get_limitAlsoAffectsType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_param0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_param0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_param1(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_param1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::IndexFilterRandomType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::IndexFilterRandomType = __cordl_object
            .invoke("get_random", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reversed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reversed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_seed(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_seed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::IndexFilterType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::IndexFilterType = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+IndexFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::IndexFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
