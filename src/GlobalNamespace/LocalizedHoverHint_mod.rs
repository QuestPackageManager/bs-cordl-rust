#[cfg(feature = "LocalizedHoverHint")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedHoverHint {
    __cordl_parent: crate::BGLib::Polyglot::LocalizedTextComponent_1<
        *mut crate::HMUI::HoverHint,
    >,
}
#[cfg(feature = "LocalizedHoverHint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalizedHoverHint => ""
    ."LocalizedHoverHint"
);
#[cfg(feature = "LocalizedHoverHint")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedHoverHint {
    type Target = crate::BGLib::Polyglot::LocalizedTextComponent_1<
        *mut crate::HMUI::HoverHint,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedHoverHint")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedHoverHint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedHoverHint")]
impl crate::GlobalNamespace::LocalizedHoverHint {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetText(
        &mut self,
        hoverHint: *mut crate::HMUI::HoverHint,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (hoverHint, value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAlignment(
        &mut self,
        hoverHint: *mut crate::HMUI::HoverHint,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAlignment", (hoverHint, direction))?;
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
#[cfg(feature = "LocalizedHoverHint")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalizedHoverHint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
