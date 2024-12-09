#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientIdentity {
    __cordl_parent: crate::System::Runtime::Remoting::Identity,
    pub _proxyReference: *mut crate::System::WeakReference,
}
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::ClientIdentity =>
    "System.Runtime.Remoting"."ClientIdentity"
);
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ClientIdentity {
    type Target = crate::System::Runtime::Remoting::Identity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ClientIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
impl crate::System::Runtime::Remoting::ClientIdentity {
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
    pub fn New(
        objectUri: *mut quest_hook::libil2cpp::Il2CppString,
        objRef: *mut crate::System::Runtime::Remoting::ObjRef,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectUri, objRef))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        objectUri: *mut quest_hook::libil2cpp::Il2CppString,
        objRef: *mut crate::System::Runtime::Remoting::ObjRef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectUri, objRef))?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::MarshalByRefObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::MarshalByRefObject = __cordl_object
            .invoke("get_ClientProxy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_TargetUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientProxy(
        &mut self,
        value: *mut crate::System::MarshalByRefObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientProxy", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ClientIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
