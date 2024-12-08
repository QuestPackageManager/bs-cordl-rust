#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct IConstructionCallMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::IConstructionCallMessage =>
    "System.Runtime.Remoting.Activation"."IConstructionCallMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::IConstructionCallMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::IConstructionCallMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
impl crate::System::Runtime::Remoting::Activation::IConstructionCallMessage {
    pub fn get_ContextProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_ContextProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ActivationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ActivationType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CallSiteActivationAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_CallSiteActivationAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ActivationTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ActivationTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Activator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Activation::IActivator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Activation::IActivator = __cordl_object
            .invoke("get_Activator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Activator(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Activation::IActivator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Activator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+IConstructionCallMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::IConstructionCallMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
