#[cfg(feature = "SonyVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyVersion {
    __cordl_parent: crate::System::Object,
    pub _majorVersion: u32,
    pub _minorVersion: u32,
}
#[cfg(feature = "SonyVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SonyVersion => ""."SonyVersion"
);
#[cfg(feature = "SonyVersion")]
impl std::ops::Deref for SonyVersion {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyVersion")]
impl std::ops::DerefMut for SonyVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyVersion")]
impl SonyVersion {
    #[cfg(feature = "SonyVersion+VersionFormat")]
    pub type VersionFormat = crate::GlobalNamespace::SonyVersion_VersionFormat;
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
    pub fn Get(
        &mut self,
        format: crate::GlobalNamespace::SonyVersion_VersionFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Get", (format))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        majorVersion: u32,
        minorVersion: u32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (majorVersion, minorVersion))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SonyVersion")]
impl quest_hook::libil2cpp::ObjectType for SonyVersion {
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
