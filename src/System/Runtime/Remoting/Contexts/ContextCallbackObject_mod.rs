#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextCallbackObject {
    __cordl_parent: crate::System::ContextBoundObject,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::ContextCallbackObject =>
    "System.Runtime.Remoting.Contexts"."ContextCallbackObject"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::ContextCallbackObject {
    type Target = crate::System::ContextBoundObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::ContextCallbackObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
impl crate::System::Runtime::Remoting::Contexts::ContextCallbackObject {
    pub fn DoCallBack(
        &mut self,
        deleg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::CrossContextDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoCallBack", (deleg))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "System+Runtime+Remoting+Contexts+ContextCallbackObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::ContextCallbackObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
