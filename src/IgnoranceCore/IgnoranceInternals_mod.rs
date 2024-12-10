#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceInternals {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::IgnoranceCore::IgnoranceInternals =>
    "IgnoranceCore"."IgnoranceInternals"
);
#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
impl std::ops::Deref for crate::IgnoranceCore::IgnoranceInternals {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
impl std::ops::DerefMut for crate::IgnoranceCore::IgnoranceInternals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
impl crate::IgnoranceCore::IgnoranceInternals {
    pub const BindAnyAddress: &'static str = "::0";
    pub const Scheme: &'static str = "enet";
    pub const Version: &'static str = "1.4.0r2 (LTS)";
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
#[cfg(feature = "IgnoranceCore+IgnoranceInternals")]
impl quest_hook::libil2cpp::ObjectType for crate::IgnoranceCore::IgnoranceInternals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
