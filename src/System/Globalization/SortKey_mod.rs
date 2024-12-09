#[cfg(feature = "System+Globalization+SortKey")]
#[repr(C)]
#[derive(Debug)]
pub struct SortKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub source: *mut quest_hook::libil2cpp::Il2CppString,
    pub key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub options: crate::System::Globalization::CompareOptions,
    pub lcid: i32,
}
#[cfg(feature = "System+Globalization+SortKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::SortKey =>
    "System.Globalization"."SortKey"
);
#[cfg(feature = "System+Globalization+SortKey")]
impl std::ops::Deref for crate::System::Globalization::SortKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+SortKey")]
impl std::ops::DerefMut for crate::System::Globalization::SortKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+SortKey")]
impl crate::System::Globalization::SortKey {
    pub fn Equals(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_3() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppString_CompareOptions_Il2CppArray2(
        localeName: *mut quest_hook::libil2cpp::Il2CppString,
        str: *mut quest_hook::libil2cpp::Il2CppString,
        options: crate::System::Globalization::CompareOptions,
        keyData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localeName, str, options, keyData))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_Il2CppString_CompareOptions0(
        lcid: i32,
        source: *mut quest_hook::libil2cpp::Il2CppString,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lcid, source, opt))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_Il2CppString_Il2CppArray_CompareOptions_i32_i32_i32_i32_i32_i32_i32_i32_1(
        lcid: i32,
        source: *mut quest_hook::libil2cpp::Il2CppString,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        opt: crate::System::Globalization::CompareOptions,
        lv1Length: i32,
        lv2Length: i32,
        lv3Length: i32,
        kanaSmallLength: i32,
        markTypeLength: i32,
        katakanaLength: i32,
        kanaWidthLength: i32,
        identLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    lcid,
                    source,
                    buffer,
                    opt,
                    lv1Length,
                    lv2Length,
                    lv3Length,
                    kanaSmallLength,
                    markTypeLength,
                    katakanaLength,
                    kanaWidthLength,
                    identLength,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppString_CompareOptions_Il2CppArray2(
        &mut self,
        localeName: *mut quest_hook::libil2cpp::Il2CppString,
        str: *mut quest_hook::libil2cpp::Il2CppString,
        options: crate::System::Globalization::CompareOptions,
        keyData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localeName, str, options, keyData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_Il2CppString_CompareOptions0(
        &mut self,
        lcid: i32,
        source: *mut quest_hook::libil2cpp::Il2CppString,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lcid, source, opt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_Il2CppString_Il2CppArray_CompareOptions_i32_i32_i32_i32_i32_i32_i32_i32_1(
        &mut self,
        lcid: i32,
        source: *mut quest_hook::libil2cpp::Il2CppString,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        opt: crate::System::Globalization::CompareOptions,
        lv1Length: i32,
        lv2Length: i32,
        lv3Length: i32,
        kanaSmallLength: i32,
        markTypeLength: i32,
        katakanaLength: i32,
        kanaWidthLength: i32,
        identLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    lcid,
                    source,
                    buffer,
                    opt,
                    lv1Length,
                    lv2Length,
                    lv3Length,
                    kanaSmallLength,
                    markTypeLength,
                    katakanaLength,
                    kanaWidthLength,
                    identLength,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_KeyData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OriginalString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_OriginalString", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+SortKey")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::SortKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
