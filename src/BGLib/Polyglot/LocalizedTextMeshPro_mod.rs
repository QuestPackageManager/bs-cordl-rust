#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextMeshPro {
    __cordl_parent: crate::BGLib::Polyglot::LocalizedTextComponent_1<
        *mut crate::TMPro::TextMeshPro,
    >,
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizedTextMeshPro =>
    "BGLib.Polyglot"."LocalizedTextMeshPro"
);
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizedTextMeshPro {
    type Target = crate::BGLib::Polyglot::LocalizedTextComponent_1<
        *mut crate::TMPro::TextMeshPro,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizedTextMeshPro {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
impl crate::BGLib::Polyglot::LocalizedTextMeshPro {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetText(
        &mut self,
        text: *mut crate::TMPro::TextMeshPro,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (text, value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAlignment(
        &mut self,
        text: *mut crate::TMPro::TextMeshPro,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAlignment", (text, direction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshPro")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizedTextMeshPro {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
