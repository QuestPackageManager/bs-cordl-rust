#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueExtensions =>
    "UnityEngine.UIElements"."StyleValueExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl crate::UnityEngine::UIElements::StyleValueExtensions {
    pub fn CopyFrom<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        other: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyFrom", (list, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn DebugString<T>(
        styleValue: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DebugString", (styleValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLength_StyleKeyword0(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLength", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLength_StyleLength1(
        styleLength: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLength", (styleLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRotate(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRotate", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToScale(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToScale", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTranslate(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTranslate", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToYogaValue(
        length: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToYogaValue", (length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
