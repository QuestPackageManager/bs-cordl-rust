#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct WellKnownServiceTypeEntry {
    __cordl_parent: crate::System::Runtime::Remoting::TypeEntry,
    pub obj_type: *mut crate::System::Type,
    pub obj_uri: *mut crate::System::String,
    pub obj_mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::WellKnownServiceTypeEntry => "System.Runtime.Remoting"
    ."WellKnownServiceTypeEntry"
);
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl std::ops::Deref for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    type Target = crate::System::Runtime::Remoting::TypeEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
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
    pub fn get_Mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::WellKnownObjectMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Remoting::WellKnownObjectMode = __cordl_object
            .invoke("get_Mode", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        objectUri: *mut crate::System::String,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName, assemblyName, objectUri, mode))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ObjectUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        typeName: *mut crate::System::String,
        assemblyName: *mut crate::System::String,
        objectUri: *mut crate::System::String,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName, assemblyName, objectUri, mode))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
