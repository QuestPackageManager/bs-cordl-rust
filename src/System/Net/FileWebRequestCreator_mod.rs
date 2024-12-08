#[cfg(feature = "System+Net+FileWebRequestCreator")]
#[repr(C)]
#[derive(Debug)]
pub struct FileWebRequestCreator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+FileWebRequestCreator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FileWebRequestCreator =>
    "System.Net"."FileWebRequestCreator"
);
#[cfg(feature = "System+Net+FileWebRequestCreator")]
impl std::ops::Deref for crate::System::Net::FileWebRequestCreator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebRequestCreator")]
impl std::ops::DerefMut for crate::System::Net::FileWebRequestCreator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebRequestCreator")]
impl crate::System::Net::FileWebRequestCreator {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Create(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::WebRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::WebRequest = __cordl_object
            .invoke("Create", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+FileWebRequestCreator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FileWebRequestCreator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
