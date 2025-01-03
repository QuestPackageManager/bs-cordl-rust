#[cfg(feature = "OVRAnchor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRAnchor {
    pub _Handle_k__BackingField: u64,
    pub _Uuid_k__BackingField: crate::System::Guid,
}
#[cfg(feature = "OVRAnchor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRAnchor => ""."OVRAnchor"
);
#[cfg(feature = "OVRAnchor")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRAnchor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRAnchor")]
impl crate::GlobalNamespace::OVRAnchor {
    pub fn CreateSpatialAnchorAsync_Pose0(
        trackingSpacePose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor>,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRAnchor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpatialAnchorAsync", (trackingSpacePose))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchorAsync_Transform_Camera1(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        centerEyeCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRAnchor>,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRAnchor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpatialAnchorAsync", (transform, centerEyeCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRAnchor0(
        &mut self,
        other: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchors(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        queryInfo: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FetchAnchors", (anchors, queryInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_IEnumerable_1_IList_1_OVRSpace_StorageLocation_f64_2(
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FetchAnchorsAsync", (uuids, anchors, location, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_IList_1_OVRSpace_StorageLocation_i32_f64_1<T>(
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        maxResults: i32,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FetchAnchorsAsync", (anchors, location, maxResults, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn FetchAnchorsAsync_OVRPlugin_SpaceComponentType_IList_1_OVRSpace_StorageLocation_i32_f64_0(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::GlobalNamespace::OVRAnchor,
            >,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        maxResults: i32,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FetchAnchorsAsync",
                (_cordl_type, anchors, location, maxResults, timeout),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetComponent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQueryInfo_IEnumerable_1_f64_1(
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQueryInfo", (uuids, location, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQueryInfo_OVRPlugin_SpaceComponentType_i32_f64_0(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        location: crate::GlobalNamespace::OVRSpace_StorageLocation,
        maxResults: i32,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQueryInfo", (_cordl_type, location, maxResults, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceQueryCompleteData(
        data: crate::GlobalNamespace::OVRDeserialize_SpaceQueryCompleteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpaceQueryCompleteData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn SupportsComponent<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SupportsComponent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetComponent<T>(
        &mut self,
        component: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetComponent",
            (component),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: u64,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle, uuid),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuid(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uuid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::GlobalNamespace::OVRAnchor,
        rhs: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::GlobalNamespace::OVRAnchor,
        rhs: crate::GlobalNamespace::OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRAnchor {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
for crate::GlobalNamespace::OVRAnchor {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
#[cfg(feature = "OVRAnchor")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor>>
for crate::GlobalNamespace::OVRAnchor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRAnchor> {
        todo!()
    }
}
