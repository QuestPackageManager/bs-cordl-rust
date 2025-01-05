#[cfg(feature = "Mono+X509Pal")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Pal {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+X509Pal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::X509Pal => "Mono"."X509Pal"
);
#[cfg(feature = "Mono+X509Pal")]
impl std::ops::Deref for crate::Mono::X509Pal {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl std::ops::DerefMut for crate::Mono::X509Pal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl crate::Mono::X509Pal {
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::X509Pal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
