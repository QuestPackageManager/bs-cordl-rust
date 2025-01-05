#[cfg(feature = "System+Text+Normalization")]
#[repr(C)]
#[derive(Debug)]
pub struct Normalization {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+Normalization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Normalization => "System.Text"
    ."Normalization"
);
#[cfg(feature = "System+Text+Normalization")]
impl std::ops::Deref for crate::System::Text::Normalization {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Normalization")]
impl std::ops::DerefMut for crate::System::Text::Normalization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Normalization")]
impl crate::System::Text::Normalization {
    pub fn CharMapIdx(cp: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CharMapIdx", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHangul(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        current: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHangul", (sb, s, current))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_Il2CppString0(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (source, start, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_StringBuilder1(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        i: i32,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (sb, i, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compose(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compose", (source, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeChar(
        sb: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        >,
        buf: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        i: i32,
        checkType: i32,
        start: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecomposeChar", (sb, buf, s, i, checkType, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decompose_ByRefMut_i32_1(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sb: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        >,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decompose", (source, sb, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decompose_i32_0(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decompose", (source, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fetch(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fetch", (sb, s, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCanonical(
        c: i32,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bufIdx: i32,
        checkType: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCanonical", (c, buf, bufIdx, checkType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCanonicalHangul(
        s: i32,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        bufIdx: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCanonicalHangul", (s, buf, bufIdx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCombiningClass(c: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCombiningClass", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimaryCompositeFromMapIndex(
        src: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimaryCompositeFromMapIndex", (src))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimaryCompositeHelperIndex(
        cp: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimaryCompositeHelperIndex", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_NormalizationForm0(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        normalizationForm: crate::System::Text::NormalizationForm,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (source, normalizationForm))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_i32_1(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (source, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn PropValue(cp: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PropValue", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn QuickCheck(
        c: char,
        _cordl_type: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Text::NormalizationCheck> {
        let __cordl_ret: crate::System::Text::NormalizationCheck = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QuickCheck", (c, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReorderCanonical(
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sb: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        >,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReorderCanonical", (src, sb, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCompose(
        i: i32,
        starter: i32,
        candidate: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCompose", (i, starter, candidate))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryComposeWithPreviousStarter(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        current: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryComposeWithPreviousStarter", (sb, s, current))?;
        Ok(__cordl_ret.into())
    }
    pub fn load_normalization_resource(
        props: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        mappedChars: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        charMapIndex: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        helperIndex: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        mapIdxToComposite: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        combiningClass: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "load_normalization_resource",
                (
                    props,
                    mappedChars,
                    charMapIndex,
                    helperIndex,
                    mapIdxToComposite,
                    combiningClass,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+Normalization")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::Normalization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
