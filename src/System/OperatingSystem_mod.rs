#[cfg(feature = "System+OperatingSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct OperatingSystem {
    __cordl_parent: crate::System::Object,
    pub _version: *mut crate::System::Version,
    pub _platform: crate::System::PlatformID,
    pub _servicePack: *mut crate::System::String,
    pub _versionString: *mut crate::System::String,
}
#[cfg(feature = "System+OperatingSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::OperatingSystem => "System"
    ."OperatingSystem"
);
#[cfg(feature = "System+OperatingSystem")]
impl std::ops::Deref for crate::System::OperatingSystem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl std::ops::DerefMut for crate::System::OperatingSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl crate::System::OperatingSystem {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Platform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::PlatformID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::PlatformID = __cordl_object
            .invoke("get_Platform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_VersionString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_VersionString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PlatformID_Version0(
        &mut self,
        platform: crate::System::PlatformID,
        version: *mut crate::System::Version,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platform, version))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        platform: crate::System::PlatformID,
        version: *mut crate::System::Version,
        servicePack: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platform, version, servicePack))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn New_PlatformID_Version0(
        platform: crate::System::PlatformID,
        version: *mut crate::System::Version,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platform, version))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        platform: crate::System::PlatformID,
        version: *mut crate::System::Version,
        servicePack: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platform, version, servicePack))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::System::OperatingSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
