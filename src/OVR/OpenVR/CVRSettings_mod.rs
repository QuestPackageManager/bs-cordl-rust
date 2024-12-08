#[cfg(feature = "OVR+OpenVR+CVRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSettings {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRSettings,
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSettings => "OVR.OpenVR"
    ."CVRSettings"
);
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl crate::OVR::OpenVR::CVRSettings {
    pub fn GetSettingsErrorNameFromEnum(
        &mut self,
        eError: crate::OVR::OpenVR::EVRSettingsError,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetSettingsErrorNameFromEnum", (eError))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFloat", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        pchValue: *mut crate::System::Text::StringBuilder,
        unValueLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetString",
                (pchSection, pchSettingsKey, pchValue, unValueLen, peError),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        flValue: f32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (pchSection, pchSettingsKey, flValue, peError))?;
        Ok(__cordl_ret)
    }
    pub fn Sync(
        &mut self,
        bForce: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Sync", (bForce, peError))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSection(
        &mut self,
        pchSection: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSection", (pchSection, peError))?;
        Ok(__cordl_ret)
    }
    pub fn SetString(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        pchValue: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetString", (pchSection, pchSettingsKey, pchValue, peError))?;
        Ok(__cordl_ret)
    }
    pub fn SetBool(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        bValue: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (pchSection, pchSettingsKey, bValue, peError))?;
        Ok(__cordl_ret)
    }
    pub fn SetInt32(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        nValue: i32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt32", (pchSection, pchSettingsKey, nValue, peError))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
    }
    pub fn GetBool(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBool", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveKeyInSection(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveKeyInSection", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt32(
        &mut self,
        pchSection: *mut crate::System::String,
        pchSettingsKey: *mut crate::System::String,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetInt32", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
