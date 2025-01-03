#[cfg(feature = "TextStyleSO")]
#[repr(C)]
#[derive(Debug)]
pub struct TextStyleSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _textStyle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TextStyle>,
}
#[cfg(feature = "TextStyleSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TextStyleSO => ""."TextStyleSO"
);
#[cfg(feature = "TextStyleSO")]
impl std::ops::Deref for crate::GlobalNamespace::TextStyleSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TextStyleSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::TextStyleSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TextStyleSO")]
impl crate::GlobalNamespace::TextStyleSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadOnlyTextStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadOnlyTextStyle,
        > = __cordl_object.invoke("get_textStyle", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TextStyleSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TextStyleSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
