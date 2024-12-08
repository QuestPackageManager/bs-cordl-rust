#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceDescription_DeviceDescriptionJson {
    pub interface: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::String,
    pub product: *mut crate::System::String,
    pub serial: *mut crate::System::String,
    pub version: *mut crate::System::String,
    pub manufacturer: *mut crate::System::String,
    pub capabilities: *mut crate::System::String,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceDescription_DeviceDescriptionJson =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceDescription/DeviceDescriptionJson"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription_DeviceDescriptionJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
)]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription_DeviceDescriptionJson {}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDeviceDescription {
    pub m_InterfaceName: *mut crate::System::String,
    pub m_DeviceClass: *mut crate::System::String,
    pub m_Manufacturer: *mut crate::System::String,
    pub m_Product: *mut crate::System::String,
    pub m_Serial: *mut crate::System::String,
    pub m_Version: *mut crate::System::String,
    pub m_Capabilities: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceDescription =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceDescription"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription {
    #[cfg(
        feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
    )]
    pub type DeviceDescriptionJson = crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription_DeviceDescriptionJson;
    pub fn set_serial(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_serial",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_product(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_product",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_interfaceName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_interfaceName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_interfaceName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interfaceName",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_deviceClass(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_deviceClass",
            (value),
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
    pub fn get_deviceClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deviceClass",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_serial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_serial",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToJson",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputDeviceDescription0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
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
    pub fn get_manufacturer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_manufacturer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_version(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_version",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_empty",
            (),
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
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_version",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_capabilities(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capabilities",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_manufacturer(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_manufacturer",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_product(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_product",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_capabilities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capabilities",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
