#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct MaskUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::MaskUtilities =>
    "UnityEngine.UI"."MaskUtilities"
);
#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
impl std::ops::Deref for crate::UnityEngine::UI::MaskUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::UI::MaskUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
impl crate::UnityEngine::UI::MaskUtilities {
    pub fn FindRootSortOverrideCanvas(
        start: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindRootSortOverrideCanvas", (start))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRectMaskForClippable(
        clippable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::IClippable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::RectMask2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::RectMask2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRectMaskForClippable", (clippable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRectMasksForClip(
        clipper: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::RectMask2D>,
        masks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UI::RectMask2D,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRectMasksForClip", (clipper, masks))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStencilDepth(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        stopAfter: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStencilDepth", (transform, stopAfter))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDescendantOrSelf(
        father: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDescendantOrSelf", (father, child))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Notify2DMaskStateChanged(
        mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Notify2DMaskStateChanged", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyStencilStateChanged(
        mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyStencilStateChanged", (mask))?;
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
#[cfg(feature = "UnityEngine+UI+MaskUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::MaskUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
