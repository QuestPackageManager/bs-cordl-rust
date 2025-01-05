#[cfg(feature = "UnityEngine+UI+Clipping")]
#[repr(C)]
#[derive(Debug)]
pub struct Clipping {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+Clipping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::Clipping => "UnityEngine.UI"
    ."Clipping"
);
#[cfg(feature = "UnityEngine+UI+Clipping")]
impl std::ops::Deref for crate::UnityEngine::UI::Clipping {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Clipping")]
impl std::ops::DerefMut for crate::UnityEngine::UI::Clipping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+Clipping")]
impl crate::UnityEngine::UI::Clipping {
    pub fn FindCullAndClipWorldRect(
        rectMaskParents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::RectMask2D>,
            >,
        >,
        validRect: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindCullAndClipWorldRect", (rectMaskParents, validRect))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+Clipping")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::Clipping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
