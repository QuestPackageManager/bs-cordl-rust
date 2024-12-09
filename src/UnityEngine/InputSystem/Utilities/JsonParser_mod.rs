#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JsonParser {
    pub m_Text: *mut crate::System::String,
    pub m_Length: i32,
    pub m_Position: i32,
    pub m_MatchAnyElementInArray: bool,
    pub m_DryRun: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::JsonParser
    => "UnityEngine.InputSystem.Utilities"."JsonParser"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::JsonParser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser")]
impl crate::UnityEngine::InputSystem::Utilities::JsonParser {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonString")]
    pub type JsonString = crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonString;
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue")]
    pub type JsonValue = crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue;
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValueType")]
    pub type JsonValueType = crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValueType;
    pub fn CurrentPropertyHasValueEqualTo(
        &mut self,
        expectedValue: crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CurrentPropertyHasValueEqualTo",
            (expectedValue),
        )?;
        Ok(__cordl_ret)
    }
    pub fn NavigateToProperty(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NavigateToProperty",
            (path),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseArrayValue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseArrayValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseBooleanValue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseBooleanValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseNullValue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseNullValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseNumber(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseNumber",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseObjectValue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseObjectValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseStringValue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseStringValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseToken(&mut self, token: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseToken",
            (token),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseValue_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseValue_ByRefMut1(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseValue",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SkipString(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipString",
            (text),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SkipToValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipToValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SkipWhitespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipWhitespace",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        json: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (json),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isAtEnd(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isAtEnd",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonString")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JsonParser_JsonString {
    pub text: crate::UnityEngine::InputSystem::Utilities::Substring,
    pub hasEscapes: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonString")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::JsonParser_JsonString =>
    "UnityEngine.InputSystem.Utilities"."JsonParser/JsonString"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonString")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonString {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonString")]
impl crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonString {
    pub fn Equals_JsonParser_JsonString0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JsonParser_JsonValue {
    pub _cordl_type: crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValueType,
    pub boolValue: bool,
    pub realValue: f64,
    pub integerValue: i64,
    pub stringValue: crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonString,
    pub arrayValue: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
    >,
    pub objectValue: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
    >,
    pub anyValue: *mut crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue =>
    "UnityEngine.InputSystem.Utilities"."JsonParser/JsonValue"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue")]
impl crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValue+__c")]
    pub type __c = crate::UnityEngine::InputSystem::Utilities::JsonValue_JsonParser___c;
    pub fn Equals_JsonParser_JsonValue0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::JsonParser_JsonValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToBoolean",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDouble",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToInteger(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToInteger",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonParser_JsonValueType {
    Any = 7i32,
    Array = 5i32,
    Bool = 1i32,
    Integer = 3i32,
    None = 0i32,
    Object = 6i32,
    Real = 2i32,
    String = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+JsonParser+JsonValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::JsonParser_JsonValueType =>
    "UnityEngine.InputSystem.Utilities"."JsonParser/JsonValueType"
);
