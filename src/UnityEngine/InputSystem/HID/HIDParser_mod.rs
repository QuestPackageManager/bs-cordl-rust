#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
#[repr(C)]
#[derive(Debug)]
pub struct HIDParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::HID::HIDParser =>
    "UnityEngine.InputSystem.HID"."HIDParser"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HIDParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HIDParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl crate::UnityEngine::InputSystem::HID::HIDParser {
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
    pub type HIDItemStateGlobal = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
    pub type HIDItemStateLocal = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
    pub type HIDItemTypeAndTag = crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
    pub type HIDReportData = crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData;
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::HID::HIDParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HIDParser_HIDItemStateGlobal {
    pub usagePage: crate::System::Nullable_1<i32>,
    pub logicalMinimum: crate::System::Nullable_1<i32>,
    pub logicalMaximum: crate::System::Nullable_1<i32>,
    pub physicalMinimum: crate::System::Nullable_1<i32>,
    pub physicalMaximum: crate::System::Nullable_1<i32>,
    pub unitExponent: crate::System::Nullable_1<i32>,
    pub unit: crate::System::Nullable_1<i32>,
    pub reportSize: crate::System::Nullable_1<i32>,
    pub reportCount: crate::System::Nullable_1<i32>,
    pub reportId: crate::System::Nullable_1<i32>,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal =>
    "UnityEngine.InputSystem.HID"."HIDParser/HIDItemStateGlobal"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateGlobal")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateGlobal {
    pub fn GetPhysicalMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPhysicalMax",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicalMin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPhysicalMin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUsagePage(
        &mut self,
        index: i32,
        localItemState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_UsagePage = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUsagePage",
            (index, localItemState),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HIDParser_HIDItemStateLocal {
    pub usage: crate::System::Nullable_1<i32>,
    pub usageMinimum: crate::System::Nullable_1<i32>,
    pub usageMaximum: crate::System::Nullable_1<i32>,
    pub designatorIndex: crate::System::Nullable_1<i32>,
    pub designatorMinimum: crate::System::Nullable_1<i32>,
    pub designatorMaximum: crate::System::Nullable_1<i32>,
    pub stringIndex: crate::System::Nullable_1<i32>,
    pub stringMinimum: crate::System::Nullable_1<i32>,
    pub stringMaximum: crate::System::Nullable_1<i32>,
    pub usageList: *mut crate::System::Collections::Generic::List_1<i32>,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal =>
    "UnityEngine.InputSystem.HID"."HIDParser/HIDItemStateLocal"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemStateLocal")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDItemStateLocal {
    pub fn GetUsage(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUsage",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUsage(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetUsage",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HIDParser_HIDItemTypeAndTag {
    Collection = 160i32,
    Delimiter = 168i32,
    DesignatorIndex = 56i32,
    DesignatorMaximum = 88i32,
    DesignatorMinimum = 72i32,
    EndCollection = 192i32,
    Feature = 176i32,
    Input = 128i32,
    LogicalMaximum = 36i32,
    LogicalMinimum = 20i32,
    Output = 144i32,
    PhysicalMaximum = 68i32,
    PhysicalMinimum = 52i32,
    Pop = 180i32,
    Push = 164i32,
    ReportCount = 148i32,
    ReportID = 132i32,
    ReportSize = 116i32,
    StringIndex = 120i32,
    StringMaximum = 152i32,
    StringMinimum = 136i32,
    Unit = 100i32,
    UnitExponent = 84i32,
    Usage = 8i32,
    UsageMaximum = 40i32,
    UsageMinimum = 24i32,
    UsagePage = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDItemTypeAndTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::HID::HIDParser_HIDItemTypeAndTag =>
    "UnityEngine.InputSystem.HID"."HIDParser/HIDItemTypeAndTag"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HIDParser_HIDReportData {
    pub reportId: i32,
    pub reportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
    pub currentBitOffset: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::HID::HIDParser_HIDReportData =>
    "UnityEngine.InputSystem.HID"."HIDParser/HIDReportData"
);
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDParser+HIDReportData")]
impl crate::UnityEngine::InputSystem::HID::HIDParser_HIDReportData {}
