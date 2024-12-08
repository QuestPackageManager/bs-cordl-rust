#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
#[repr(C)]
#[derive(Debug)]
pub struct CodePointIndexer {
    __cordl_parent: crate::System::Object,
    pub ranges: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Mono::Globalization::Unicode::CodePointIndexer_TableRange,
    >,
    pub TotalCount: i32,
    pub defaultIndex: i32,
    pub defaultCP: i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Globalization::Unicode::CodePointIndexer
    => "Mono.Globalization.Unicode"."CodePointIndexer"
);
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::CodePointIndexer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::CodePointIndexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
impl crate::Mono::Globalization::Unicode::CodePointIndexer {
    #[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer+TableRange")]
    pub type TableRange = crate::Mono::Globalization::Unicode::CodePointIndexer_TableRange;
    pub fn _ctor(
        &mut self,
        starts: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        ends: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        defaultIndex: i32,
        defaultCP: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (starts, ends, defaultIndex, defaultCP))?;
        Ok(__cordl_ret)
    }
    pub fn ToIndex(&mut self, cp: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToIndex", (cp))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        starts: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        ends: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        defaultIndex: i32,
        defaultCP: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (starts, ends, defaultIndex, defaultCP))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::CodePointIndexer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer+TableRange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CodePointIndexer_TableRange {
    pub Start: i32,
    pub End: i32,
    pub Count: i32,
    pub IndexStart: i32,
    pub IndexEnd: i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer+TableRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::CodePointIndexer_TableRange =>
    "Mono.Globalization.Unicode"."CodePointIndexer/TableRange"
);
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer+TableRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Globalization::Unicode::CodePointIndexer_TableRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+CodePointIndexer+TableRange")]
impl crate::Mono::Globalization::Unicode::CodePointIndexer_TableRange {
    pub fn _ctor(
        &mut self,
        start: i32,
        end: i32,
        indexStart: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (start, end, indexStart),
        )?;
        Ok(__cordl_ret)
    }
}
