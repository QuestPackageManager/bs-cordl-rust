#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientIdentity {
    __cordl_parent: crate::System::Runtime::Remoting::Identity,
    pub _proxyReference: quest_hook::libil2cpp::Gc<crate::System::WeakReference>,
}
#[cfg(feature = "System+Runtime+Remoting+ClientIdentity")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::ClientIdentity {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "ClientIdentity";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        requestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = __cordl_object.invoke("CreateObjRef", (requestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectUri, objRef))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectUri, objRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClientProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject> = __cordl_object
            .invoke("get_ClientProxy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TargetUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ClientProxy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientProxy", (value))?;
        Ok(__cordl_ret.into())
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
