#[cfg(feature = "Mono+X509Pal")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Pal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+X509Pal")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::X509Pal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "X509Pal";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Mono+X509Pal")]
impl std::ops::Deref for crate::Mono::X509Pal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
