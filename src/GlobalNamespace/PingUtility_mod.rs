#[cfg(feature = "PingUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct PingUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PingUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PingUtility => ""."PingUtility"
);
#[cfg(feature = "PingUtility")]
impl std::ops::Deref for crate::GlobalNamespace::PingUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PingUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::PingUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PingUtility")]
impl crate::GlobalNamespace::PingUtility {
    pub fn PingAsync(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i64>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i64>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("PingAsync", (url))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PingUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PingUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
