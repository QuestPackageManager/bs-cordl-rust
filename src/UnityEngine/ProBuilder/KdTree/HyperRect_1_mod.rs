#[cfg(feature = "UnityEngine+ProBuilder+KdTree+HyperRect_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HyperRect_1<T: quest_hook::libil2cpp::Type> {
    pub minPoint: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    pub maxPoint: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+HyperRect_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::KdTree::HyperRect_1 < T
    > => "UnityEngine.ProBuilder.KdTree"."HyperRect`1<T>" < T >
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+HyperRect_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::KdTree::HyperRect_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+HyperRect_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::HyperRect_1<T> {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::KdTree::HyperRect_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ProBuilder::KdTree::HyperRect_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clone",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClosestPoint(
        &mut self,
        toPoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        math: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetClosestPoint",
            (toPoint, math),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_MaxPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_MinPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxPoint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_MaxPoint",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MinPoint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_MinPoint",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
