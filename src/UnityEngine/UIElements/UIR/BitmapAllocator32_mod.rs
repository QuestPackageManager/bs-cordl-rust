#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitmapAllocator32 {
    pub m_PageHeight: i32,
    pub m_Pages: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32_Page,
        >,
    >,
    pub m_AllocMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<u32>,
    >,
    pub m_EntryWidth: i32,
    pub m_EntryHeight: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::BitmapAllocator32
    => "UnityEngine.UIElements.UIR"."BitmapAllocator32"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::BitmapAllocator32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32")]
impl crate::UnityEngine::UIElements::UIR::BitmapAllocator32 {
    #[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32+Page")]
    pub type Page = crate::UnityEngine::UIElements::UIR::BitmapAllocator32_Page;
    pub fn Allocate(
        &mut self,
        storage: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (storage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Construct(
        &mut self,
        pageHeight: i32,
        entryWidth: i32,
        entryHeight: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Construct",
            (pageHeight, entryWidth, entryHeight),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CountTrailingZeroes(val: u32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CountTrailingZeroes", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceFirstAlloc(
        &mut self,
        firstPageX: u16,
        firstPageY: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ForceFirstAlloc",
            (firstPageX, firstPageY),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Free",
            (alloc),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllocPageAtlasLocation(
        &mut self,
        page: i32,
        x: quest_hook::libil2cpp::ByRefMut<u16>,
        y: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAllocPageAtlasLocation",
            (page, x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_entryHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_entryHeight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_entryWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_entryWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32+Page")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitmapAllocator32_Page {
    pub x: u16,
    pub y: u16,
    pub freeSlots: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32+Page")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::BitmapAllocator32_Page =>
    "UnityEngine.UIElements.UIR"."BitmapAllocator32/Page"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32+Page")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::BitmapAllocator32_Page {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BitmapAllocator32+Page")]
impl crate::UnityEngine::UIElements::UIR::BitmapAllocator32_Page {}
