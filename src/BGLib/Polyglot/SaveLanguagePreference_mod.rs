#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
#[repr(C)]
#[derive(Debug)]
pub struct SaveLanguagePreference {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub preferenceKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::SaveLanguagePreference =>
    "BGLib.Polyglot"."SaveLanguagePreference"
);
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl std::ops::Deref for crate::BGLib::Polyglot::SaveLanguagePreference {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::SaveLanguagePreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl crate::BGLib::Polyglot::SaveLanguagePreference {
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
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::Polyglot::SaveLanguagePreference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl AsRef<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::SaveLanguagePreference {
    fn as_ref(&self) -> &crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+Polyglot+SaveLanguagePreference")]
impl AsMut<crate::BGLib::Polyglot::ILocalize>
for crate::BGLib::Polyglot::SaveLanguagePreference {
    fn as_mut(&mut self) -> &mut crate::BGLib::Polyglot::ILocalize {
        unsafe { std::mem::transmute(self) }
    }
}
