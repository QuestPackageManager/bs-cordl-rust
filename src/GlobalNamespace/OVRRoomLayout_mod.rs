#[cfg(feature = "OVRRoomLayout")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRRoomLayout {
    pub _Handle_k__BackingField: u64,
}
#[cfg(feature = "OVRRoomLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OVRRoomLayout => ""."OVRRoomLayout"
);
#[cfg(feature = "OVRRoomLayout")]
unsafe impl quest_hook::libil2cpp::ThisArgument for OVRRoomLayout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRRoomLayout")]
impl OVRRoomLayout {
    pub fn Equals_OVRRoomLayout0(
        &mut self,
        other: OVRRoomLayout,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FetchLayoutAnchorsAsync(
        &mut self,
        anchors: *mut crate::System::Collections::Generic::List_1<OVRAnchor>,
    ) -> quest_hook::libil2cpp::Result<OVRTask_1<bool>> {
        let __cordl_ret: OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FetchLayoutAnchorsAsync",
            (anchors),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__FromAnchor(
        &mut self,
        anchor: OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<OVRRoomLayout> {
        let __cordl_ret: OVRRoomLayout = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRRoomLayout>.FromAnchor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__SetEnabledAsync(
        &mut self,
        enabled: bool,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<OVRTask_1<bool>> {
        let __cordl_ret: OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRRoomLayout>.SetEnabledAsync",
            (enabled, timeout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRRoomLayout>.get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IOVRAnchorComponent_OVRRoomLayout__get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IOVRAnchorComponent<OVRRoomLayout>.get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetRoomLayout(
        &mut self,
        ceiling: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        floor: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
        walls: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetRoomLayout",
            (ceiling, floor, walls),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        anchor: OVRAnchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (anchor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEnabled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
}