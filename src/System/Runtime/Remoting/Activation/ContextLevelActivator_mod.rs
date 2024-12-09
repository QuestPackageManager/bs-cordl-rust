#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextLevelActivator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_NextActivator: *mut crate::System::Runtime::Remoting::Activation::IActivator,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::ContextLevelActivator =>
    "System.Runtime.Remoting.Activation"."ContextLevelActivator"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::ContextLevelActivator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::ContextLevelActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
impl crate::System::Runtime::Remoting::Activation::ContextLevelActivator {
    pub fn Activate(
        &mut self,
        ctorCall: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage = __cordl_object
            .invoke("Activate", (ctorCall))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        next: *mut crate::System::Runtime::Remoting::Activation::IActivator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        next: *mut crate::System::Runtime::Remoting::Activation::IActivator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (next))?;
        Ok(__cordl_ret)
    }
    pub fn get_NextActivator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Activation::IActivator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Activation::IActivator = __cordl_object
            .invoke("get_NextActivator", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ContextLevelActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::ContextLevelActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
