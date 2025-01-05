#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParserAdapterWithValidation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdParserAdapterWithValidation =>
    "System.Xml"."IDtdParserAdapterWithValidation"
);
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl std::ops::Deref for crate::System::Xml::IDtdParserAdapterWithValidation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParserAdapterWithValidation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl crate::System::Xml::IDtdParserAdapterWithValidation {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_DtdValidation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DtdValidation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidationEventHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::IValidationEventHandling>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::IValidationEventHandling,
        > = __cordl_object.invoke("get_ValidationEventHandling", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::IDtdParserAdapterWithValidation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl AsRef<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterWithValidation {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterWithValidation")]
impl AsMut<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterWithValidation {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
