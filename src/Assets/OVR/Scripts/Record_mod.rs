#[cfg(feature = "Assets+OVR+Scripts+Record")]
#[repr(C)]
#[derive(Debug)]
pub struct Record {
    __cordl_parent: crate::System::Object,
    pub sortOrder: i32,
    pub category: *mut crate::System::String,
    pub message: *mut crate::System::String,
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Assets::OVR::Scripts::Record =>
    "Assets.OVR.Scripts"."Record"
);
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl std::ops::Deref for crate::Assets::OVR::Scripts::Record {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl std::ops::DerefMut for crate::Assets::OVR::Scripts::Record {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl crate::Assets::OVR::Scripts::Record {
    pub fn New(
        order: i32,
        cat: *mut crate::System::String,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (order, cat, msg))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        order: i32,
        cat: *mut crate::System::String,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (order, cat, msg))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl quest_hook::libil2cpp::ObjectType for crate::Assets::OVR::Scripts::Record {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
