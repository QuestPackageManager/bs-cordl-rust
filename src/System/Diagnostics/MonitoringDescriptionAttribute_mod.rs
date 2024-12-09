#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MonitoringDescriptionAttribute {
    __cordl_parent: crate::System::ComponentModel::DescriptionAttribute,
}
#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::MonitoringDescriptionAttribute => "System.Diagnostics"
    ."MonitoringDescriptionAttribute"
);
#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::MonitoringDescriptionAttribute {
    type Target = crate::System::ComponentModel::DescriptionAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
impl std::ops::DerefMut for crate::System::Diagnostics::MonitoringDescriptionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
impl crate::System::Diagnostics::MonitoringDescriptionAttribute {
    pub fn New(
        description: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (description))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        description: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (description))?;
        Ok(__cordl_ret)
    }
    pub fn get_Description(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Description", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Diagnostics+MonitoringDescriptionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::MonitoringDescriptionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
