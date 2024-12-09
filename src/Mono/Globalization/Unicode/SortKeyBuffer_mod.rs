#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct SortKeyBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub l1b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l2b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l3b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l4sb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l4tb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l4kb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l4wb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub l5b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub source: *mut quest_hook::libil2cpp::Il2CppString,
    pub l1: i32,
    pub l2: i32,
    pub l3: i32,
    pub l4s: i32,
    pub l4t: i32,
    pub l4k: i32,
    pub l4w: i32,
    pub l5: i32,
    pub lcid: i32,
    pub options: crate::System::Globalization::CompareOptions,
    pub processLevel2: bool,
    pub frenchSort: bool,
    pub frenchSorted: bool,
}
#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Globalization::Unicode::SortKeyBuffer =>
    "Mono.Globalization.Unicode"."SortKeyBuffer"
);
#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::SortKeyBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::SortKeyBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
impl crate::Mono::Globalization::Unicode::SortKeyBuffer {
    pub fn AppendBufferPrimitive(
        &mut self,
        value: u8,
        buf: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        bidx: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendBufferPrimitive", (value, buf, bidx))?;
        Ok(__cordl_ret)
    }
    pub fn AppendCJKExtension(
        &mut self,
        lv1msb: u8,
        lv1lsb: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendCJKExtension", (lv1msb, lv1lsb))?;
        Ok(__cordl_ret)
    }
    pub fn AppendKana(
        &mut self,
        category: u8,
        lv1: u8,
        lv2: u8,
        lv3: u8,
        isSmallKana: bool,
        markType: u8,
        isKatakana: bool,
        isHalfWidth: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AppendKana",
                (category, lv1, lv2, lv3, isSmallKana, markType, isKatakana, isHalfWidth),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AppendLevel5(
        &mut self,
        category: u8,
        lv1: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendLevel5", (category, lv1))?;
        Ok(__cordl_ret)
    }
    pub fn AppendNormal(
        &mut self,
        category: u8,
        lv1: u8,
        lv2: u8,
        lv3: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendNormal", (category, lv1, lv2, lv3))?;
        Ok(__cordl_ret)
    }
    pub fn GetOptimizedLength(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        len: i32,
        defaultValue: u8,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetOptimizedLength", (data, len, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetResultAndReset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetResultAndReset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        options: crate::System::Globalization::CompareOptions,
        lcid: i32,
        s: *mut quest_hook::libil2cpp::Il2CppString,
        frenchSort: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (options, lcid, s, frenchSort))?;
        Ok(__cordl_ret)
    }
    pub fn New(lcid: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lcid))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lcid: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lcid))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SortKeyBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::SortKeyBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
