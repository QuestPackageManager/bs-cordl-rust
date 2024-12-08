#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::IBeat =>
    "BeatmapSaveDataCommon"."IBeat"
);
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl std::ops::Deref for crate::BeatmapSaveDataCommon::IBeat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl std::ops::DerefMut for crate::BeatmapSaveDataCommon::IBeat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl crate::BeatmapSaveDataCommon::IBeat {
    pub fn System_IComparable_BeatmapSaveDataCommon_IBeat__CompareTo(
        &mut self,
        other: *mut crate::BeatmapSaveDataCommon::IBeat,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.IComparable<BeatmapSaveDataCommon.IBeat>.CompareTo",
                (other),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataCommon::IBeat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
