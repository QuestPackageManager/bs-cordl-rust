#[cfg(feature = "cordl_class_System+Text+UTF8Encoding+UTF8EncodingSealed")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8Encoding_UTF8EncodingSealed {
    __cordl_parent: crate::System::Text::UTF8Encoding,
}
#[cfg(feature = "cordl_class_System+Text+UTF8Encoding+UTF8EncodingSealed")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text";
    const CLASS_NAME: &'static str = "UTF8Encoding/UTF8EncodingSealed";
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
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl std::ops::Deref for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    type Target = crate::System::Text::UTF8Encoding;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl std::ops::DerefMut for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    pub fn New(
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoderShouldEmitUTF8Identifier))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (encoderShouldEmitUTF8Identifier))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Preamble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<u8>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::ReadOnlySpan_1<u8>,
                        0usize,
                    >("get_Preamble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Preamble", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Text+UTF8Encoding+UTF8EncodingSealed")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
