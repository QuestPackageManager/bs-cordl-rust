#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteActivator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::RemoteActivator =>
    "System.Runtime.Remoting.Activation"."RemoteActivator"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Activation::RemoteActivator {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::RemoteActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl crate::System::Runtime::Remoting::Activation::RemoteActivator {
    pub fn Activate(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
        > = __cordl_object.invoke("Activate", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextActivator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        > = __cordl_object.invoke("get_NextActivator", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::RemoteActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Activation::IActivator>,
> for crate::System::Runtime::Remoting::Activation::RemoteActivator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Activation::IActivator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivator")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Activation::IActivator>,
> for crate::System::Runtime::Remoting::Activation::RemoteActivator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Activation::IActivator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
