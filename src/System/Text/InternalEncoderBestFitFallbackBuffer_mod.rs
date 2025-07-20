#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalEncoderBestFitFallbackBuffer {
    __cordl_parent: crate::System::Text::EncoderFallbackBuffer,
    pub _cBestFit: char,
    pub _oFallback: quest_hook::libil2cpp::Gc<
        crate::System::Text::InternalEncoderBestFitFallback,
    >,
    pub _iCount: i32,
    pub _iSize: i32,
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text";
    const CLASS_NAME: &'static str = "InternalEncoderBestFitFallbackBuffer";
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
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl std::ops::Deref for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    type Target = crate::System::Text::EncoderFallbackBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl std::ops::DerefMut for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    pub fn Fallback__cordl_char_i32_1(
        &mut self,
        charUnknownHigh: char,
        charUnknownLow: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(char, char, i32), bool, 3usize>("Fallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "Fallback", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (charUnknownHigh, charUnknownLow, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Fallback_i32_0(
        &mut self,
        charUnknown: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(char, i32), bool, 2usize>("Fallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "Fallback", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (charUnknown, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), char, 0usize>("GetNextChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "GetNextChar", 0usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn MovePrevious(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("MovePrevious")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "MovePrevious", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalEncoderBestFitFallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fallback))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryBestFit(&mut self, cUnknown: char) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(char), char, 1usize>("TryBestFit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "TryBestFit", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, (cUnknown))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalEncoderBestFitFallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Text::InternalEncoderBestFitFallback,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (fallback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSyncObject() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_InternalSyncObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "get_InternalSyncObject",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Text::InternalEncoderBestFitFallbackBuffer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Remaining")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Text::InternalEncoderBestFitFallbackBuffer as
                    quest_hook::libil2cpp::Type > ::class(), "get_Remaining", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
