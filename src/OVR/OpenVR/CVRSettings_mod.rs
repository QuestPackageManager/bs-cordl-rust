#[cfg(feature = "OVR+OpenVR+CVRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRSettings,
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSettings => "OVR.OpenVR"
    ."CVRSettings"
);
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetBool(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBool", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFloat(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFloat", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInt32(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetInt32", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSettingsErrorNameFromEnum(
        &mut self,
        eError: crate::OVR::OpenVR::EVRSettingsError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSettingsErrorNameFromEnum", (eError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveKeyInSection(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveKeyInSection", (pchSection, pchSettingsKey, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSection(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSection", (pchSection, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBool(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bValue: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBool", (pchSection, pchSettingsKey, bValue, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFloat(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flValue: f32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (pchSection, pchSettingsKey, flValue, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInt32(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nValue: i32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt32", (pchSection, pchSettingsKey, nValue, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetString(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetString", (pchSection, pchSettingsKey, pchValue, peError))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
