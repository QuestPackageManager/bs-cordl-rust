#[cfg(feature = "IReadOnlyTextStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadOnlyTextStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IReadOnlyTextStyle")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IReadOnlyTextStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IReadOnlyTextStyle";
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
#[cfg(feature = "IReadOnlyTextStyle")]
impl std::ops::Deref for crate::GlobalNamespace::IReadOnlyTextStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IReadOnlyTextStyle")]
impl std::ops::DerefMut for crate::GlobalNamespace::IReadOnlyTextStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IReadOnlyTextStyle")]
impl crate::GlobalNamespace::IReadOnlyTextStyle {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
#[cfg(feature = "IReadOnlyTextStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IReadOnlyTextStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
