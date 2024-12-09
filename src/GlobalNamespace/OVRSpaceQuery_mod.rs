#[cfg(feature = "OVRSpaceQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSpaceQuery {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSpaceQuery")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpaceQuery => ""
    ."OVRSpaceQuery"
);
#[cfg(feature = "OVRSpaceQuery")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSpaceQuery {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpaceQuery")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSpaceQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpaceQuery")]
impl crate::GlobalNamespace::OVRSpaceQuery {
    #[cfg(feature = "OVRSpaceQuery+Options")]
    pub type Options = crate::GlobalNamespace::OVRSpaceQuery_Options;
}
#[cfg(feature = "OVRSpaceQuery")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSpaceQuery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSpaceQuery+Options")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRSpaceQuery_Options {
    pub _MaxResults_k__BackingField: i32,
    pub _Timeout_k__BackingField: f64,
    pub _Location_k__BackingField: crate::GlobalNamespace::OVRSpace_StorageLocation,
    pub _QueryType_k__BackingField: crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    pub _ActionType_k__BackingField: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    pub _componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    pub _uuidFilter: *mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Guid,
    >,
}
#[cfg(feature = "OVRSpaceQuery+Options")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpaceQuery_Options => ""
    ."OVRSpaceQuery/Options"
);
#[cfg(feature = "OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpaceQuery_Options {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpaceQuery+Options")]
impl crate::GlobalNamespace::OVRSpaceQuery_Options {
    pub const MaxUuidCount: i32 = 1024i32;
    pub fn ToQueryInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToQueryInfo",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryQuerySpaces(
        &mut self,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryQuerySpaces",
            (requestId),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        other: crate::GlobalNamespace::OVRSpaceQuery_Options,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ActionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ActionType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ComponentFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ComponentFilter",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSpace_StorageLocation,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRSpace_StorageLocation = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Location",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxResults(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MaxResults",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_QueryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_QueryType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Timeout",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_UuidFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::System::Guid,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_UuidFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ActionType(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ActionType",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_ComponentFilter(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ComponentFilter",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Location(
        &mut self,
        value: crate::GlobalNamespace::OVRSpace_StorageLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Location",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_MaxResults(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_MaxResults",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_QueryType(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_QueryType",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Timeout(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Timeout",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_UuidFilter(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::System::Guid,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_UuidFilter",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
