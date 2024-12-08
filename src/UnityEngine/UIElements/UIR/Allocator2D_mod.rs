#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Allocator2D_Alloc2D {
    pub rect: crate::UnityEngine::RectInt,
    pub row: *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D => "UnityEngine.UIElements.UIR"
    ."Allocator2D/Alloc2D"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    pub fn _ctor(
        &mut self,
        row: *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
        alloc: crate::UnityEngine::UIElements::UIR::Alloc,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (row, alloc, width, height),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Allocator2D {
    __cordl_parent: crate::System::Object,
    pub m_MinSize: crate::UnityEngine::Vector2Int,
    pub m_MaxSize: crate::UnityEngine::Vector2Int,
    pub m_MaxAllocSize: crate::UnityEngine::Vector2Int,
    pub m_RowHeightBias: i32,
    pub m_Rows: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    >,
    pub m_Areas: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Allocator2D =>
    "UnityEngine.UIElements.UIR"."Allocator2D"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D {
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
    pub type Row = crate::UnityEngine::UIElements::UIR::Allocator2D_Row;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
    pub type Alloc2D = crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
    pub type Area = crate::UnityEngine::UIElements::UIR::Allocator2D_Area;
    pub fn Free(
        &mut self,
        alloc2D: crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Free", (alloc2D))?;
        Ok(__cordl_ret)
    }
    pub fn TryAllocate(
        &mut self,
        width: i32,
        height: i32,
        alloc2D: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAllocate", (width, height, alloc2D))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minSize, maxSize, rowHeightBias))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minSize, maxSize, rowHeightBias))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Allocator2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
#[repr(C)]
#[derive(Debug)]
pub struct Allocator2D_Area {
    __cordl_parent: crate::System::Object,
    pub rect: crate::UnityEngine::RectInt,
    pub allocator: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Allocator2D_Area
    => "UnityEngine.UIElements.UIR"."Allocator2D/Area"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    pub fn _ctor(
        &mut self,
        rect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rect))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        rect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rect))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
#[repr(C)]
#[derive(Debug)]
pub struct Allocator2D_Row {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    >,
    pub rect: crate::UnityEngine::RectInt,
    pub area: *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
    pub allocator: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator,
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
    pub next: *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Allocator2D_Row =>
    "UnityEngine.UIElements.UIR"."Allocator2D/Row"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
