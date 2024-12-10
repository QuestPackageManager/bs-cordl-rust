#[cfg(feature = "TMPro+KerningTable")]
#[repr(C)]
#[derive(Debug)]
pub struct KerningTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub kerningPairs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::KerningPair,
    >,
}
#[cfg(feature = "TMPro+KerningTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::KerningTable => "TMPro"."KerningTable"
);
#[cfg(feature = "TMPro+KerningTable")]
impl std::ops::Deref for crate::TMPro::KerningTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+KerningTable")]
impl std::ops::DerefMut for crate::TMPro::KerningTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+KerningTable")]
impl crate::TMPro::KerningTable {
    #[cfg(feature = "TMPro+KerningTable+__c")]
    pub type __c = crate::TMPro::KerningTable___c;
    #[cfg(feature = "TMPro+KerningTable+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::TMPro::KerningTable___c__DisplayClass3_0;
    #[cfg(feature = "TMPro+KerningTable+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::TMPro::KerningTable___c__DisplayClass4_0;
    #[cfg(feature = "TMPro+KerningTable+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::TMPro::KerningTable___c__DisplayClass5_0;
    pub fn AddGlyphPairAdjustmentRecord(
        &mut self,
        first: u32,
        firstAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
        second: u32,
        secondAdjustments: crate::TMPro::GlyphValueRecord_Legacy,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "AddGlyphPairAdjustmentRecord",
                (first, firstAdjustments, second, secondAdjustments),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddKerningPair_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKerningPair", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddKerningPair_u32_u32_f32_1(
        &mut self,
        first: u32,
        second: u32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("AddKerningPair", (first, second, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveKerningPair_i32_0(
        &mut self,
        left: i32,
        right: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveKerningPair", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveKerningPair_i32_1(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveKerningPair", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortKerningPairs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortKerningPairs", ())?;
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
#[cfg(feature = "TMPro+KerningTable")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::KerningTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
