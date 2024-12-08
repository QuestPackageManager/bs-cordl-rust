#[cfg(feature = "System+Threading+SparselyPopulatedArrayAddInfo_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SparselyPopulatedArrayAddInfo_1<T: quest_hook::libil2cpp::Type> {
    pub _source: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    pub _index: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayAddInfo_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::SparselyPopulatedArrayAddInfo_1 < T > => "System.Threading"
    ."SparselyPopulatedArrayAddInfo`1<T>" < T >
);
#[cfg(feature = "System+Threading+SparselyPopulatedArrayAddInfo_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::SparselyPopulatedArrayAddInfo_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayAddInfo_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::SparselyPopulatedArrayAddInfo_1<T> {
    pub fn _ctor(
        &mut self,
        source: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (source, index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Index",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Source(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<
            T,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Source", ())?;
        Ok(__cordl_ret)
    }
}
