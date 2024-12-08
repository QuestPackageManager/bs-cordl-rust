#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteActivationAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _contextProperties: *mut crate::System::Collections::IList,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::RemoteActivationAttribute =>
    "System.Runtime.Remoting.Activation"."RemoteActivationAttribute"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    pub fn _ctor(
        &mut self,
        contextProperties: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contextProperties))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertiesForNewContext(
        &mut self,
        ctor: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPropertiesForNewContext", (ctor))?;
        Ok(__cordl_ret)
    }
    pub fn IsContextOK(
        &mut self,
        ctx: *mut crate::System::Runtime::Remoting::Contexts::Context,
        ctor: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContextOK", (ctx, ctor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        contextProperties: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contextProperties))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
