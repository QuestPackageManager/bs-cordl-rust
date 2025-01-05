#[cfg(feature = "System+Text+EncodingHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+EncodingHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::EncodingHelper => "System.Text"
    ."EncodingHelper"
);
#[cfg(feature = "System+Text+EncodingHelper")]
impl std::ops::Deref for crate::System::Text::EncodingHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingHelper")]
impl std::ops::DerefMut for crate::System::Text::EncodingHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingHelper")]
impl crate::System::Text::EncodingHelper {
    pub fn GetDefaultEncoding() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultEncoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCodePage(
        code_page: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCodePage", (code_page))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeI18N(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeI18N", (name, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UTF8Unmarked() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UTF8Unmarked", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+EncodingHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::EncodingHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
