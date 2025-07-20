#[cfg(feature = "System+Text+EncodingProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+EncodingProvider")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Text::EncodingProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text";
    const CLASS_NAME: &'static str = "EncodingProvider";
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
#[cfg(feature = "System+Text+EncodingProvider")]
impl std::ops::Deref for crate::System::Text::EncodingProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl std::ops::DerefMut for crate::System::Text::EncodingProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl crate::System::Text::EncodingProvider {
    pub fn GetEncodingFromProvider_Il2CppString1(
        encodingName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                1usize,
            >("GetEncodingFromProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncodingFromProvider", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked((), (encodingName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncodingFromProvider_i32_0(
        codepage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                1usize,
            >("GetEncodingFromProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncodingFromProvider", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked((), (codepage))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncodingFromProvider_i32_EncoderFallback_DecoderFallback2(
        codepage: i32,
        enc: quest_hook::libil2cpp::Gc<crate::System::Text::EncoderFallback>,
        dec: quest_hook::libil2cpp::Gc<crate::System::Text::DecoderFallback>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::EncoderFallback>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::DecoderFallback>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                3usize,
            >("GetEncodingFromProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncodingFromProvider", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked((), (codepage, enc, dec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoding_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                1usize,
            >("GetEncoding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncoding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoding_i32_1(
        &mut self,
        codepage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                1usize,
            >("GetEncoding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncoding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked(self, (codepage))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoding_i32_EncoderFallback_DecoderFallback2(
        &mut self,
        codepage: i32,
        encoderFallback: quest_hook::libil2cpp::Gc<crate::System::Text::EncoderFallback>,
        decoderFallback: quest_hook::libil2cpp::Gc<crate::System::Text::DecoderFallback>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::EncodingProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Text::EncoderFallback>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::DecoderFallback>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                3usize,
            >("GetEncoding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::EncodingProvider as quest_hook::libil2cpp::Type
                    > ::class(), "GetEncoding", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            method.invoke_unchecked(self, (codepage, encoderFallback, decoderFallback))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::EncodingProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
