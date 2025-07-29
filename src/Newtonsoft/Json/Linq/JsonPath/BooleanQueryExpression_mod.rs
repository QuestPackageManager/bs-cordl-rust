#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct BooleanQueryExpression {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression,
    pub Left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Linq.JsonPath";
    const CLASS_NAME: &'static str = "BooleanQueryExpression";
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
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    type Target = crate::Newtonsoft::Json::Linq::JsonPath::QueryExpression;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    pub fn EqualsWithStrictMatch(
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        queryValue: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                        ),
                        bool,
                        2usize,
                    >("EqualsWithStrictMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EqualsWithStrictMatch", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (value, queryValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EqualsWithStringCoercion(
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        queryValue: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                        ),
                        bool,
                        2usize,
                    >("EqualsWithStringCoercion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EqualsWithStringCoercion", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (value, queryValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetResult(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Newtonsoft::Json::Linq::JToken,
                                >,
                            >,
                        >,
                        3usize,
                    >("GetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetResult", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (root, t, o))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        t: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JsonSelectSettings,
                            >,
                        ),
                        bool,
                        3usize,
                    >("IsMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsMatch",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (root, t, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchTokens(
        &mut self,
        leftResult: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        rightResult: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JToken,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JsonSelectSettings,
                            >,
                        ),
                        bool,
                        3usize,
                    >("MatchTokens")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MatchTokens", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (leftResult, rightResult, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_operator, left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn RegexEquals(
        input: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        pattern: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JValue>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JValue,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::Linq::JsonSelectSettings,
                            >,
                        ),
                        bool,
                        3usize,
                    >("RegexEquals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegexEquals", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (input, pattern, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_operator: crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Newtonsoft::Json::Linq::JsonPath::QueryOperator,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_operator, left, right))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+JsonPath+BooleanQueryExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::BooleanQueryExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
