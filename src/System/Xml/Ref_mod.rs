#[cfg(feature = "System+Xml+Ref")]
#[repr(C)]
#[derive(Debug)]
pub struct Ref {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+Ref")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Ref => "System.Xml"."Ref"
);
#[cfg(feature = "System+Xml+Ref")]
impl std::ops::Deref for crate::System::Xml::Ref {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ref")]
impl std::ops::DerefMut for crate::System::Xml::Ref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ref")]
impl crate::System::Xml::Ref {
    pub fn Equal(
        strA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        strB: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equal", (strA, strB))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Ref")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Ref {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
