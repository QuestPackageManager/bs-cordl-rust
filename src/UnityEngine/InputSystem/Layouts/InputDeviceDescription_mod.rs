#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDeviceDescription {
    pub m_InterfaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_DeviceClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Manufacturer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Product: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Serial: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Capabilities: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub fn ComparePropertyToDeviceDescriptor(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propertyValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deviceDescriptor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ComparePropertyToDeviceDescriptor",
                (propertyName, propertyValue, deviceDescriptor),
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
    pub fn Equals_InputDeviceDescription0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromJson", (json))?;
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
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToJson", ())?;
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
    pub fn get_capabilities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_capabilities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceClass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_deviceClass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_empty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_empty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interfaceName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_interfaceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_manufacturer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_manufacturer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_product(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_product", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_serial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_serial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        right: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_capabilities(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capabilities",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceClass(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_deviceClass",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interfaceName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_interfaceName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_manufacturer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_manufacturer",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_product(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_product",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_serial(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_serial",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_version(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_version",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    >,
> for crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    >,
> for crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Layouts+InputDeviceDescription+DeviceDescriptionJson"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputDeviceDescription_DeviceDescriptionJson {
    pub interface: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub product: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub serial: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub manufacturer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub capabilities: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
