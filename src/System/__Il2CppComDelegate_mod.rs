#[cfg(feature = "System+__Il2CppComDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct __Il2CppComDelegate {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::__Il2CppComObject>,
}
#[cfg(feature = "System+__Il2CppComDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::__Il2CppComDelegate => "System"
    ."__Il2CppComDelegate"
);
#[cfg(feature = "System+__Il2CppComDelegate")]
impl std::ops::Deref for crate::System::__Il2CppComDelegate {
    type Target = quest_hook::libil2cpp::Gc<crate::System::__Il2CppComObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+__Il2CppComDelegate")]
impl std::ops::DerefMut for crate::System::__Il2CppComDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+__Il2CppComDelegate")]
impl crate::System::__Il2CppComDelegate {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+__Il2CppComDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::__Il2CppComDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
