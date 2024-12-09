#[cfg(feature = "EnvironmentSizeData")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentSizeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _floorType: crate::GlobalNamespace::EnvironmentSizeData_FloorType,
    pub _ceilingType: crate::GlobalNamespace::EnvironmentSizeData_CeilingType,
    pub _trackLaneType: crate::GlobalNamespace::EnvironmentSizeData_TrackLaneType,
}
#[cfg(feature = "EnvironmentSizeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentSizeData => ""
    ."EnvironmentSizeData"
);
#[cfg(feature = "EnvironmentSizeData")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentSizeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentSizeData")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentSizeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentSizeData")]
impl crate::GlobalNamespace::EnvironmentSizeData {
    #[cfg(feature = "EnvironmentSizeData+CeilingType")]
    pub type CeilingType = crate::GlobalNamespace::EnvironmentSizeData_CeilingType;
    #[cfg(feature = "EnvironmentSizeData+FloorType")]
    pub type FloorType = crate::GlobalNamespace::EnvironmentSizeData_FloorType;
    #[cfg(feature = "EnvironmentSizeData+TrackLaneType")]
    pub type TrackLaneType = crate::GlobalNamespace::EnvironmentSizeData_TrackLaneType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_ceilingType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentSizeData_CeilingType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentSizeData_CeilingType = __cordl_object
            .invoke("get_ceilingType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_floorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentSizeData_FloorType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentSizeData_FloorType = __cordl_object
            .invoke("get_floorType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackLaneType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentSizeData_TrackLaneType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentSizeData_TrackLaneType = __cordl_object
            .invoke("get_trackLaneType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EnvironmentSizeData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EnvironmentSizeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentSizeData+CeilingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentSizeData_CeilingType {
    LowCeiling = 1i32,
    NoCeiling = 0i32,
}
#[cfg(feature = "EnvironmentSizeData+CeilingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentSizeData_CeilingType
    => ""."EnvironmentSizeData/CeilingType"
);
#[cfg(feature = "EnvironmentSizeData+FloorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentSizeData_FloorType {
    CloseTo0 = 1i32,
    NoFloor = 0i32,
}
#[cfg(feature = "EnvironmentSizeData+FloorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentSizeData_FloorType
    => ""."EnvironmentSizeData/FloorType"
);
#[cfg(feature = "EnvironmentSizeData+TrackLaneType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentSizeData_TrackLaneType {
    None = 0i32,
    Normal = 1i32,
}
#[cfg(feature = "EnvironmentSizeData+TrackLaneType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnvironmentSizeData_TrackLaneType => ""
    ."EnvironmentSizeData/TrackLaneType"
);
