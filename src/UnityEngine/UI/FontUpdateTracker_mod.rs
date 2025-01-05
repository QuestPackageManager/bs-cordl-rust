#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
#[repr(C)]
#[derive(Debug)]
pub struct FontUpdateTracker {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::FontUpdateTracker =>
    "UnityEngine.UI"."FontUpdateTracker"
);
#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
impl std::ops::Deref for crate::UnityEngine::UI::FontUpdateTracker {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
impl std::ops::DerefMut for crate::UnityEngine::UI::FontUpdateTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
impl crate::UnityEngine::UI::FontUpdateTracker {
    pub fn RebuildForFont(
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildForFont", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackText(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrackText", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn UntrackText(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UntrackText", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+FontUpdateTracker")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::FontUpdateTracker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
