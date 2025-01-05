#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumFieldHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EnumFieldHelpers =>
    "UnityEngine.UIElements"."EnumFieldHelpers"
);
#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EnumFieldHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EnumFieldHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
impl crate::UnityEngine::UIElements::EnumFieldHelpers {
    pub fn ExtractValue(
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
        resEnumType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        resEnumValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Enum>,
        >,
        resIncludeObsoleteValues: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractValue",
                (bag, cc, resEnumType, resEnumValue, resIncludeObsoleteValues),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumFieldHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EnumFieldHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
