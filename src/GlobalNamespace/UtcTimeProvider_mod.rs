#[cfg(feature = "UtcTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct UtcTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UtcTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UtcTimeProvider => ""
    ."UtcTimeProvider"
);
#[cfg(feature = "UtcTimeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::UtcTimeProvider {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UtcTimeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::UtcTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UtcTimeProvider")]
impl crate::GlobalNamespace::UtcTimeProvider {
    pub fn GetTicks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetTicks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetTimeMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UtcTimeProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UtcTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UtcTimeProvider")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>>
for crate::GlobalNamespace::UtcTimeProvider {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UtcTimeProvider")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>>
for crate::GlobalNamespace::UtcTimeProvider {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider> {
        unsafe { std::mem::transmute(self) }
    }
}
