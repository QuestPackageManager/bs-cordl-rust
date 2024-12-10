#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HoudiniVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HoudiniVersion =>
    "HoudiniEngineUnity"."HEU_HoudiniVersion"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HoudiniVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HoudiniVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
impl crate::HoudiniEngineUnity::HEU_HoudiniVersion {
    pub const HAPI_BIN_PATH: &'static str = "/bin";
    pub const HAPI_LIBRARY: &'static str = "/opt/hfs18.5.633/dsolib/libHAPIL.so";
    pub const HAPI_LIBRARY_PATH: &'static str = "/dsolib";
    pub const HAPI_SERVER: &'static str = "/opt/hfs18.5.633/bin/HARS";
    pub const HARC_LIBRARY: &'static str = "/opt/hfs18.5.633/dsolib/libHARC.so";
    pub const HOUDINI_BUILD: i32 = 633i32;
    pub const HOUDINI_ENGINE_API: i32 = 3i32;
    pub const HOUDINI_ENGINE_MAJOR: i32 = 3i32;
    pub const HOUDINI_ENGINE_MINOR: i32 = 6i32;
    pub const HOUDINI_INSTALL_PATH: &'static str = "/opt/hfs18.5.633";
    pub const HOUDINI_MAJOR: i32 = 18i32;
    pub const HOUDINI_MINOR: i32 = 5i32;
    pub const HOUDINI_PATCH: i32 = 0i32;
    pub const HOUDINI_VERSION_STRING: &'static str = "18.5.633";
    pub const UNITY_PLUGIN_VERSION: i32 = 2i32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HoudiniVersion")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_HoudiniVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
