#[cfg(feature = "TextStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct TextStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fontStyle: crate::TMPro::FontStyles,
    pub _fontSize: f32,
    pub _autoSizing: bool,
    pub _fontSizeMin: f32,
    pub _fontSizeMax: f32,
    pub _charWidthMaxAdj: f32,
    pub _lineSpacingMax: f32,
}
#[cfg(feature = "TextStyle")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::TextStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TextStyle";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "TextStyle")]
impl std::ops::Deref for crate::GlobalNamespace::TextStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TextStyle")]
impl std::ops::DerefMut for crate::GlobalNamespace::TextStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TextStyle")]
impl crate::GlobalNamespace::TextStyle {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        fontStyle: crate::TMPro::FontStyles,
        fontSize: f32,
        autoSizing: bool,
        fontSizeMin: f32,
        fontSizeMax: f32,
        charWidthMaxAdj: f32,
        lineSpacingMax: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Update",
                (
                    fontStyle,
                    fontSize,
                    autoSizing,
                    fontSizeMin,
                    fontSizeMax,
                    charWidthMaxAdj,
                    lineSpacingMax,
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
    pub fn get_autoSizing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoSizing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_charWidthMaxAdj(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_charWidthMaxAdj", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSizeMax(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSizeMax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSizeMin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fontSizeMin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::FontStyles> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::FontStyles = __cordl_object
            .invoke("get_fontStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineSpacingMax(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lineSpacingMax", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TextStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TextStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TextStyle")]
impl AsRef<crate::GlobalNamespace::IReadOnlyTextStyle>
for crate::GlobalNamespace::TextStyle {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReadOnlyTextStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TextStyle")]
impl AsMut<crate::GlobalNamespace::IReadOnlyTextStyle>
for crate::GlobalNamespace::TextStyle {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReadOnlyTextStyle {
        unsafe { std::mem::transmute(self) }
    }
}
