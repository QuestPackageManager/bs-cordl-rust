#[cfg(feature = "MidiParser+MidiEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MidiEvent {
    pub AbsoluteTicksTime: i32,
    pub Type: u8,
    pub Arg1: i32,
    pub Arg2: i32,
    pub Arg3: i32,
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::Type for crate::MidiParser::MidiEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "MidiParser";
    const CLASS_NAME: &'static str = "MidiEvent";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::Argument for crate::MidiParser::MidiEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::MidiParser::MidiEvent {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::Returned for crate::MidiParser::MidiEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::Return for crate::MidiParser::MidiEvent {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::MidiParser::MidiEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MidiParser+MidiEvent")]
impl crate::MidiParser::MidiEvent {
    pub fn get_Channel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MidiParser::MidiEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Channel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MidiParser::MidiEvent as quest_hook::libil2cpp::Type >
                    ::class(), "get_Channel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MetaEventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MidiParser::MetaEventType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MidiParser::MidiEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::MidiParser::MetaEventType,
                0usize,
            >("get_MetaEventType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MidiParser::MidiEvent as quest_hook::libil2cpp::Type >
                    ::class(), "get_MetaEventType", 0usize
                )
            });
        let __cordl_ret: crate::MidiParser::MetaEventType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Note(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MidiParser::MidiEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Note")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MidiParser::MidiEvent as quest_hook::libil2cpp::Type >
                    ::class(), "get_Note", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MidiParser::MidiEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MidiParser::MidiEvent as quest_hook::libil2cpp::Type >
                    ::class(), "get_Value", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Velocity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::MidiParser::MidiEvent as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Velocity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::MidiParser::MidiEvent as quest_hook::libil2cpp::Type >
                    ::class(), "get_Velocity", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
