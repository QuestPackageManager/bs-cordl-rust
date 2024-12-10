#[cfg(feature = "System+OperatingSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct OperatingSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: *mut crate::System::Version,
    pub _platform: crate::System::PlatformID,
    pub _servicePack: *mut quest_hook::libil2cpp::Il2CppString,
    pub _versionString: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+OperatingSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::OperatingSystem => "System"
    ."OperatingSystem"
);
#[cfg(feature = "System+OperatingSystem")]
impl std::ops::Deref for crate::System::OperatingSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString1(
        platform: crate::System::PlatformID,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
        servicePack: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platform, version, servicePack))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PlatformID_Version0(
        platform: crate::System::PlatformID,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platform, version))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        platform: crate::System::PlatformID,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
        servicePack: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platform, version, servicePack))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PlatformID_Version0(
        &mut self,
        platform: crate::System::PlatformID,
        version: quest_hook::libil2cpp::Gc<crate::System::Version>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platform, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Platform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::PlatformID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::PlatformID = __cordl_object
            .invoke("get_Platform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_VersionString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_VersionString", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+OperatingSystem")]
impl AsRef<crate::System::ICloneable> for crate::System::OperatingSystem {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl AsMut<crate::System::ICloneable> for crate::System::OperatingSystem {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::OperatingSystem {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+OperatingSystem")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::OperatingSystem {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
