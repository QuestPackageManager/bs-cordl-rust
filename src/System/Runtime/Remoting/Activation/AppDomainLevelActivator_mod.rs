#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomainLevelActivator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Activation::IActivator,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::AppDomainLevelActivator =>
    "System.Runtime.Remoting.Activation"."AppDomainLevelActivator"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    pub fn Activate(
        &mut self,
        ctorCall: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("Activate", (ctorCall))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (activationUrl, next))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (activationUrl, next))?;
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
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl AsRef<crate::System::Runtime::Remoting::Activation::IActivator>
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Activation::IActivator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl AsMut<crate::System::Runtime::Remoting::Activation::IActivator>
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Activation::IActivator {
        unsafe { std::mem::transmute(self) }
    }
}
