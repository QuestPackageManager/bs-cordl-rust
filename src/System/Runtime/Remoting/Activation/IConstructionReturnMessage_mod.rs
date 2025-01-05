#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct IConstructionReturnMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::IConstructionReturnMessage =>
    "System.Runtime.Remoting.Activation"."IConstructionReturnMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    >,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    >,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    >,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionReturnMessage")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    >,
> for crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
