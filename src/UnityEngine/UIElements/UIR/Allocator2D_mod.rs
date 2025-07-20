#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Allocator2D {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MinSize: crate::UnityEngine::Vector2Int,
    pub m_MaxSize: crate::UnityEngine::Vector2Int,
    pub m_MaxAllocSize: crate::UnityEngine::Vector2Int,
    pub m_RowHeightBias: i32,
    pub m_Rows: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
            >,
        >,
    >,
    pub m_Areas: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Allocator2D {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "Allocator2D";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D {
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
    pub type Alloc2D = crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
    pub type Area = crate::UnityEngine::UIElements::UIR::Allocator2D_Area;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
    pub type Row = crate::UnityEngine::UIElements::UIR::Allocator2D_Row;
    pub fn BuildAreas(
        areas: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
                >,
            >,
        >,
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
                                    >,
                                >,
                            >,
                            crate::UnityEngine::Vector2Int,
                            crate::UnityEngine::Vector2Int,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("BuildAreas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildAreas", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (areas, minSize, maxSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRowArray(
        maxRowHeight: i32,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                                >,
                            >,
                        >,
                        2usize,
                    >("BuildRowArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildRowArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                >,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (maxRowHeight, rowHeightBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeMaxAllocSize(
        areas: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
                >,
            >,
        >,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
                                    >,
                                >,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::Vector2Int,
                        2usize,
                    >("ComputeMaxAllocSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeMaxAllocSize", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int = unsafe {
            cordl_method_info.invoke_unchecked((), (areas, rowHeightBias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        &mut self,
        alloc2D: crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Free")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Free",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc2D))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minSize, maxSize, rowHeightBias))?;
        Ok(__cordl_object.into())
    }
    pub fn TryAllocate(
        &mut self,
        width: i32,
        height: i32,
        alloc2D: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryAllocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAllocate", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (width, height, alloc2D))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        minSize: crate::UnityEngine::Vector2Int,
        maxSize: crate::UnityEngine::Vector2Int,
        rowHeightBias: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector2Int,
                            crate::UnityEngine::Vector2Int,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (minSize, maxSize, rowHeightBias))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Allocator2D_Alloc2D {
    pub rect: crate::UnityEngine::RectInt,
    pub row: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    >,
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "Allocator2D/Alloc2D";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Alloc2D")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::Allocator2D_Alloc2D {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
        row: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
        >,
        alloc: crate::UnityEngine::UIElements::UIR::Alloc,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                            >,
                            crate::UnityEngine::UIElements::UIR::Alloc,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (row, alloc, width, height))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
#[repr(C)]
#[derive(Debug)]
pub struct Allocator2D_Area {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rect: crate::UnityEngine::RectInt,
    pub allocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BestFitAllocator,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "Allocator2D/Area";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Area")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D_Area {
    pub fn New(
        rect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rect))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        rect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::RectInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rect))?
        };
        Ok(__cordl_ret.into())
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
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Allocator2D_Row>,
    >,
    pub rect: crate::UnityEngine::RectInt,
    pub area: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::Allocator2D_Area,
    >,
    pub allocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BestFitAllocator,
    >,
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
    pub next: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "Allocator2D/Row";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Allocator2D_Row>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Allocator2D+Row")]
impl crate::UnityEngine::UIElements::UIR::Allocator2D_Row {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Allocator2D_Row>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                        >,
                        0usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        row: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Allocator2D_Row,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
