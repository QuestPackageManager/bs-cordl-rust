#[cfg(feature = "System+NotImplemented")]
#[repr(C)]
#[derive(Debug)]
pub struct NotImplemented {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+NotImplemented")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::NotImplemented => "System"
    ."NotImplemented"
);
#[cfg(feature = "System+NotImplemented")]
impl std::ops::Deref for crate::System::NotImplemented {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+NotImplemented")]
impl std::ops::DerefMut for crate::System::NotImplemented {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+NotImplemented")]
impl crate::System::NotImplemented {
    pub fn get_ByDesign() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ByDesign", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+NotImplemented")]
impl quest_hook::libil2cpp::ObjectType for crate::System::NotImplemented {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
