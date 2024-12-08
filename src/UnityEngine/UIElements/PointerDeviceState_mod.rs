#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+LocationFlag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerDeviceState_LocationFlag {
    None = 0i32,
    OutsidePanel = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+LocationFlag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::PointerDeviceState_LocationFlag =>
    "UnityEngine.UIElements"."PointerDeviceState/LocationFlag"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerDeviceState {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerDeviceState =>
    "UnityEngine.UIElements"."PointerDeviceState"
);
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerDeviceState {
    type Target = crate::System::Object;
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
#[cfg(feature = "UnityEngine+UIElements+PointerDeviceState+PointerLocation")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PointerDeviceState_PointerLocation {
    pub _Position_k__BackingField: crate::UnityEngine::Vector2,
    pub _Panel_k__BackingField: *mut crate::UnityEngine::UIElements::IPanel,
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
        panel: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLocation",
            (position, panel),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_Panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::IPanel> {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IPanel = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Panel",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Panel(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::IPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Panel",
            (value),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
