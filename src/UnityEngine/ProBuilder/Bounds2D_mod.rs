#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Bounds2D {
    __cordl_parent: crate::System::Object,
    pub center: crate::UnityEngine::Vector2,
    pub m_Size: crate::UnityEngine::Vector2,
    pub m_Extents: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Bounds2D =>
    "UnityEngine.ProBuilder"."Bounds2D"
);
#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Bounds2D {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Bounds2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
impl crate::UnityEngine::ProBuilder::Bounds2D {
    pub fn ContainsPoint(
        &mut self,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsPoint", (point))?;
        Ok(__cordl_ret)
    }
    pub fn IntersectsLineSegment(
        &mut self,
        lineStart: crate::UnityEngine::Vector2,
        lineEnd: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IntersectsLineSegment", (lineStart, lineEnd))?;
        Ok(__cordl_ret)
    }
    pub fn Intersects_Bounds2D0(
        &mut self,
        bounds: *mut crate::UnityEngine::ProBuilder::Bounds2D,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Intersects", (bounds))?;
        Ok(__cordl_ret)
    }
    pub fn Intersects_Rect1(
        &mut self,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Intersects", (rect))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IList_1_2(
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points))?;
        Ok(__cordl_object)
    }
    pub fn New_IList_1_IList_1_3(
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
        indexes: *mut crate::System::Collections::Generic::IList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points, indexes))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_Il2CppArray4(
        points: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        edges: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points, edges))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_5(
        points: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (points, length))?;
        Ok(__cordl_object)
    }
    pub fn New_Vector2_Vector2_1(
        center: crate::UnityEngine::Vector2,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (center, _cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn SetWithPoints_IList_1_0(
        &mut self,
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWithPoints", (points))?;
        Ok(__cordl_ret)
    }
    pub fn SetWithPoints_IList_1_1(
        &mut self,
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
        indexes: *mut crate::System::Collections::Generic::IList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWithPoints", (points, indexes))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_1_2(
        &mut self,
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_1_IList_1_3(
        &mut self,
        points: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::Vector2,
        >,
        indexes: *mut crate::System::Collections::Generic::IList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points, indexes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray4(
        &mut self,
        points: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        edges: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::ProBuilder::Edge,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points, edges))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i32_5(
        &mut self,
        points: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (points, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector2_Vector2_1(
        &mut self,
        center: crate::UnityEngine::Vector2,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (center, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_corners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector2,
        > = __cordl_object.invoke("get_corners", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_extents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_extents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_size", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_size(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Bounds2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Bounds2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
