#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct CADSerializer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CADSerializer =>
    "System.Runtime.Remoting.Channels"."CADSerializer"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::CADSerializer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Channels::CADSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl crate::System::Runtime::Remoting::Channels::CADSerializer {
    pub fn DeserializeMessage(
        mem: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeMessage", (mem, msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObject(
        mem: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObject", (mem))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeObjectSafe(
        mem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeObjectSafe", (mem))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeMessage(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeObject", (obj))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CADSerializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::CADSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
