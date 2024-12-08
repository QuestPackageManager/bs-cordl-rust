#[cfg(feature = "ENet+Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Native {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ENet+Native")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::Native => "ENet"."Native"
);
#[cfg(feature = "ENet+Native")]
impl std::ops::Deref for crate::ENet::Native {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Native")]
impl std::ops::DerefMut for crate::ENet::Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Native")]
impl crate::ENet::Native {
    pub const cryptoNativeLibrary: &'static str = "crypto";
    pub const nativeLibrary: &'static str = "enet";
    pub const sslNativeLibrary: &'static str = "ssl";
}
#[cfg(feature = "ENet+Native")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
