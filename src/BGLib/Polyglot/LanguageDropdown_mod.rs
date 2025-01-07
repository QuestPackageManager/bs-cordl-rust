#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageDropdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub dropdown: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Dropdown>,
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LanguageDropdown {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LanguageDropdown";
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
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl std::ops::Deref for crate::BGLib::Polyglot::LanguageDropdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LanguageDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl crate::BGLib::Polyglot::LanguageDropdown {
    pub fn CreateDropdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateDropdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnLocalize(
        &mut self,
        localization: quest_hook::libil2cpp::Gc<
            crate::BGLib::Polyglot::LocalizationModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLocalize", (localization))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LanguageDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl AsRef<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::LanguageDropdown {
    fn as_ref(&self) -> &crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageDropdown")]
impl AsMut<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::LanguageDropdown {
    fn as_mut(&mut self) -> &mut crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
