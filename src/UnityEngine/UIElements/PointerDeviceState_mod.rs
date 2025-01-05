#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerDeviceState {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerDeviceState =>
    "UnityEngine.UIElements"."PointerDeviceState"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerDeviceState {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerDeviceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
impl crate::UnityEngine::UIElements::PointerDeviceState {
    #[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+LocationFlag")]
    pub type LocationFlag = crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag;
    #[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
    pub type PointerLocation = crate::UnityEngine::UIElements::PointerDeviceState_PointerLocation;
    pub fn GetPanel(
        pointerId: i32,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPanel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPanel", (pointerId, contextType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerPanelWithSoftPointerCapture(
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPanel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayerPanelWithSoftPointerCapture", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPointerPosition(
        pointerId: i32,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPointerPosition", (pointerId, contextType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPressedButtons(pointerId: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPressedButtons", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAdditionalPressedButtons(
        pointerId: i32,
        exceptButtonId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAdditionalPressedButtons", (pointerId, exceptButtonId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlagFast(
        flagSet: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
        flag: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasFlagFast", (flagSet, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLocationFlag(
        pointerId: i32,
        contextType: crate::UnityEngine::UIElements::ContextType,
        flag: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasLocationFlag", (pointerId, contextType, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn PressButton(
        pointerId: i32,
        buttonId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PressButton", (pointerId, buttonId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseAllButtons(
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseAllButtons", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseButton(
        pointerId: i32,
        buttonId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseButton", (pointerId, buttonId))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePanelData(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemovePanelData", (panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SavePointerPosition(
        pointerId: i32,
        position: crate::UnityEngine::Vector2,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SavePointerPosition", (pointerId, position, panel, contextType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerPanelWithSoftPointerCapture(
        pointerId: i32,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPlayerPanelWithSoftPointerCapture", (pointerId, panel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerDeviceState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+LocationFlag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PointerDeviceState_LocationFlag {
    #[default]
    None = 0i32,
    OutsidePanel = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+LocationFlag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::PointerDeviceState_LocationFlag =>
    "UnityEngine.UIElements"."PointerDeviceState/LocationFlag"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PointerDeviceState_PointerLocation {
    pub _Position_k__BackingField: crate::UnityEngine::Vector2,
    pub _Panel_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IPanel,
    >,
    pub _Flags_k__BackingField: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::PointerDeviceState_PointerLocation =>
    "UnityEngine.UIElements"."PointerDeviceState/PointerLocation"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::PointerDeviceState_PointerLocation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
impl crate::UnityEngine::UIElements::PointerDeviceState_PointerLocation {
    pub fn SetLocation(
        &mut self,
        position: crate::UnityEngine::Vector2,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLocation",
            (position, panel),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Flags",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPanel,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Panel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Flags(
        &mut self,
        value: crate::UnityEngine::UIElements::PointerDeviceState_LocationFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Flags",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Panel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Panel",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
