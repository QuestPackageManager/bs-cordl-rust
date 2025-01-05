#[cfg(feature = "ColorSchemeTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemeTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _colorSchemeView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSchemeView,
    >,
    pub _editIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
}
#[cfg(feature = "ColorSchemeTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorSchemeTableCell => ""
    ."ColorSchemeTableCell"
);
#[cfg(feature = "ColorSchemeTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::ColorSchemeTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorSchemeTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorSchemeTableCell")]
impl crate::GlobalNamespace::ColorSchemeTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetColors(
        &mut self,
        saberAColor: crate::UnityEngine::Color,
        saberBColor: crate::UnityEngine::Color,
        environment0Color: crate::UnityEngine::Color,
        environment1Color: crate::UnityEngine::Color,
        environmentColor0Boost: crate::UnityEngine::Color,
        environmentColor1Boost: crate::UnityEngine::Color,
        obstacleColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetColors",
                (
                    saberAColor,
                    saberBColor,
                    environment0Color,
                    environment1Color,
                    environmentColor0Boost,
                    environmentColor1Boost,
                    obstacleColor,
                ),
            )?;
        Ok(__cordl_ret.into())
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
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_text", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showEditIcon(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showEditIcon", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_text(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorSchemeTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorSchemeTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
