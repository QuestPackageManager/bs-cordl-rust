#[cfg(feature = "System+BitConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BitConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+BitConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::BitConverter => "System"."BitConverter"
);
#[cfg(feature = "System+BitConverter")]
impl std::ops::Deref for crate::System::BitConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+BitConverter")]
impl std::ops::DerefMut for crate::System::BitConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+BitConverter")]
impl crate::System::BitConverter {
    #[cfg(feature = "System+BitConverter+__c")]
    pub type __c = crate::System::BitConverter___c;
}
#[cfg(feature = "System+BitConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::BitConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}