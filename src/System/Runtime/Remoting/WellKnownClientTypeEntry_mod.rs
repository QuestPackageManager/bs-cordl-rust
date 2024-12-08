#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct WellKnownClientTypeEntry {
    __cordl_parent: crate::System::Runtime::Remoting::TypeEntry,
    pub obj_type: *mut crate::System::Type,
    pub obj_url: *mut crate::System::String,
    pub app_url: *mut crate::System::String,
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::WellKnownClientTypeEntry => "System.Runtime.Remoting"
    ."WellKnownClientTypeEntry"
);
#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
impl std::ops::Deref for crate::System::Runtime::Remoting::WellKnownClientTypeEntry {
    type Target = crate::System::Runtime::Remoting::TypeEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::WellKnownClientTypeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
impl crate::System::Runtime::Remoting::WellKnownClientTypeEntry {
    pub fn _ctor(
        &mut self,
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        objectUrl: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName, assemblyName, objectUrl))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ObjectType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ApplicationUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ApplicationUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ObjectUrl", ())?;
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
    pub fn New(
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        objectUrl: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName, assemblyName, objectUrl))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownClientTypeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::WellKnownClientTypeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
