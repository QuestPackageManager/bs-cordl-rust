#[cfg(feature = "StaticPacketPoolProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticPacketPoolProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "StaticPacketPoolProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StaticPacketPoolProvider => ""
    ."StaticPacketPoolProvider"
);
#[cfg(feature = "StaticPacketPoolProvider")]
impl std::ops::Deref for crate::GlobalNamespace::StaticPacketPoolProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPacketPoolProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::StaticPacketPoolProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPacketPoolProvider")]
impl crate::GlobalNamespace::StaticPacketPoolProvider {
    pub fn GetPacketPool<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PacketPool_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPacketPool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPacketPool(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        pool: quest_hook::libil2cpp::ByRefMut<*mut crate::GlobalNamespace::IPacketPool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetPacketPool", (t, pool))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StaticPacketPoolProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticPacketPoolProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
