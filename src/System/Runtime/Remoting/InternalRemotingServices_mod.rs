#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalRemotingServices {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::InternalRemotingServices => "System.Runtime.Remoting"
    ."InternalRemotingServices"
);
#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::InternalRemotingServices {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::InternalRemotingServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
impl crate::System::Runtime::Remoting::InternalRemotingServices {
    pub fn GetCachedSoapAttribute(
        reflectionObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Metadata::SoapAttribute,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Metadata::SoapAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedSoapAttribute", (reflectionObject))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+InternalRemotingServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::InternalRemotingServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
