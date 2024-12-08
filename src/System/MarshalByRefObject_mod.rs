#[cfg(feature = "System+MarshalByRefObject")]
#[repr(C)]
#[derive(Debug)]
pub struct MarshalByRefObject {
    __cordl_parent: crate::System::Object,
    pub _identity: *mut crate::System::Object,
}
#[cfg(feature = "System+MarshalByRefObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MarshalByRefObject => "System"
    ."MarshalByRefObject"
);
#[cfg(feature = "System+MarshalByRefObject")]
impl std::ops::Deref for crate::System::MarshalByRefObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MarshalByRefObject")]
impl std::ops::DerefMut for crate::System::MarshalByRefObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MarshalByRefObject")]
impl crate::System::MarshalByRefObject {
    pub fn CreateObjRef(
        &mut self,
        requestedType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Runtime::Remoting::ObjRef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::ObjRef = __cordl_object
            .invoke("CreateObjRef", (requestedType))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeLifetimeService(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("InitializeLifetimeService", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_ObjectIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::ServerIdentity,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::ServerIdentity = __cordl_object
            .invoke("get_ObjectIdentity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ObjectIdentity(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::ServerIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectIdentity", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+MarshalByRefObject")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MarshalByRefObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}