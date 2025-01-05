#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
#[repr(C)]
#[derive(Debug)]
pub struct TernaryTreeReadOnly {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub nodeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::TernaryTreeReadOnly => "System.Xml"
    ."TernaryTreeReadOnly"
);
#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
impl std::ops::Deref for crate::System::Xml::TernaryTreeReadOnly {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
impl std::ops::DerefMut for crate::System::Xml::TernaryTreeReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
impl crate::System::Xml::TernaryTreeReadOnly {
    pub fn FindCaseInsensitiveString(
        &mut self,
        stringToFind: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object
            .invoke("FindCaseInsensitiveString", (stringToFind))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nodeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nodeBuffer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nodeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nodeBuffer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+TernaryTreeReadOnly")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::TernaryTreeReadOnly {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
