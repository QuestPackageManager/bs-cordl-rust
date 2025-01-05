#[cfg(feature = "RpcPool")]
#[repr(C)]
#[derive(Debug)]
pub struct RpcPool {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "RpcPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RpcPool => ""."RpcPool"
);
#[cfg(feature = "RpcPool")]
impl std::ops::Deref for crate::GlobalNamespace::RpcPool {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RpcPool")]
impl std::ops::DerefMut for crate::GlobalNamespace::RpcPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RpcPool")]
impl crate::GlobalNamespace::RpcPool {
    pub fn Fill<T>() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fill", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Obtain<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Obtain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        t: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRemoteProcedureCall>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RpcPool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RpcPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
