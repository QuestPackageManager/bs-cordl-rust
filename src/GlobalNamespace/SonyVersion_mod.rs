#[cfg(feature = "SonyVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _majorVersion: u32,
    pub _minorVersion: u32,
}
#[cfg(feature = "SonyVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyVersion => ""."SonyVersion"
);
#[cfg(feature = "SonyVersion")]
impl std::ops::Deref for crate::GlobalNamespace::SonyVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyVersion")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyVersion")]
impl crate::GlobalNamespace::SonyVersion {
    #[cfg(feature = "SonyVersion+VersionFormat")]
    pub type VersionFormat = crate::GlobalNamespace::SonyVersion_VersionFormat;
    pub fn Get(
        &mut self,
        format: crate::GlobalNamespace::SonyVersion_VersionFormat,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("Get", (format))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        majorVersion: u32,
        minorVersion: u32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (majorVersion, minorVersion))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        majorVersion: u32,
        minorVersion: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (majorVersion, minorVersion))?;
        Ok(__cordl_ret)
    }
    pub fn get_majorVersion(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_majorVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minorVersion(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_minorVersion", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyVersion")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SonyVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyVersion+VersionFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SonyVersion_VersionFormat {
    FullLong = 0i32,
    FullShort = 2i32,
    Long = 1i32,
    Short = 3i32,
}
#[cfg(feature = "SonyVersion+VersionFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyVersion_VersionFormat => ""
    ."SonyVersion/VersionFormat"
);
