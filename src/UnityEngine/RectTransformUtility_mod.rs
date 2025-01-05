#[cfg(feature = "UnityEngine+RectTransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct RectTransformUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RectTransformUtility =>
    "UnityEngine"."RectTransformUtility"
);
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::Deref for crate::UnityEngine::RectTransformUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::RectTransformUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl crate::UnityEngine::RectTransformUtility {
    pub fn FlipLayoutAxes(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        keepPositioning: bool,
        recursive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlipLayoutAxes", (rect, keepPositioning, recursive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlipLayoutOnAxis(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        axis: i32,
        keepPositioning: bool,
        recursive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlipLayoutOnAxis", (rect, axis, keepPositioning, recursive))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransposed(
        input: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTransposed", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustPoint(
        point: crate::UnityEngine::Vector2,
        elementTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PixelAdjustPoint", (point, elementTransform, canvas))?;
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustPoint_Injected(
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        elementTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PixelAdjustPoint_Injected",
                (point, elementTransform, canvas, ret),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustRect(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PixelAdjustRect", (rectTransform, canvas))?;
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustRect_Injected(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PixelAdjustRect_Injected", (rectTransform, canvas, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInRectangle(
        screenPoint: crate::UnityEngine::Vector2,
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInRectangle", (screenPoint, rect, cam, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInRectangle_Injected(
        screenPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInRectangle_Injected", (screenPoint, rect, cam, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RectangleContainsScreenPoint_Gc_Vector2_Gc0(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectangleContainsScreenPoint", (rect, screenPoint, cam))?;
        Ok(__cordl_ret.into())
    }
    pub fn RectangleContainsScreenPoint_Vector4_1(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectangleContainsScreenPoint", (rect, screenPoint, cam, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToLocalPointInRectangle(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        localPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScreenPointToLocalPointInRectangle",
                (rect, screenPoint, cam, localPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        screenPos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_ret: crate::UnityEngine::Ray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScreenPointToRay", (cam, screenPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToWorldPointInRectangle(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScreenPointToWorldPointInRectangle",
                (rect, screenPoint, cam, worldPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WorldToScreenPoint", (cam, worldPoint))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RectTransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
