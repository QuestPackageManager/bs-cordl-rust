#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
#[repr(C)]
#[derive(Debug)]
pub struct CADObjRef {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub objref: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    pub SourceDomain: i32,
    pub TypeInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Messaging::CADObjRef
    => "System.Runtime.Remoting.Messaging"."CADObjRef"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::CADObjRef {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::CADObjRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
impl crate::System::Runtime::Remoting::Messaging::CADObjRef {
    pub fn New(
        o: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        sourceDomain: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o, sourceDomain))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        sourceDomain: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o, sourceDomain))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CADObjRef")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::CADObjRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
