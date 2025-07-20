#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonLoadSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _commentHandling: crate::Newtonsoft::Json::Linq::CommentHandling,
    pub _lineInfoHandling: crate::Newtonsoft::Json::Linq::LineInfoHandling,
    pub _duplicatePropertyNameHandling: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Linq";
    const CLASS_NAME: &'static str = "JsonLoadSettings";
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
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CommentHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Linq::CommentHandling> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Newtonsoft::Json::Linq::CommentHandling,
                        0usize,
                    >("get_CommentHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CommentHandling", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Linq::CommentHandling = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DuplicatePropertyNameHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
                        0usize,
                    >("get_DuplicatePropertyNameHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DuplicatePropertyNameHandling", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_LineInfoHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Linq::LineInfoHandling> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Newtonsoft::Json::Linq::LineInfoHandling,
                        0usize,
                    >("get_LineInfoHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_LineInfoHandling", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Newtonsoft::Json::Linq::LineInfoHandling = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_CommentHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::CommentHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Newtonsoft::Json::Linq::CommentHandling),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_CommentHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_CommentHandling", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_DuplicatePropertyNameHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_DuplicatePropertyNameHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_DuplicatePropertyNameHandling", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_LineInfoHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::LineInfoHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Newtonsoft::Json::Linq::LineInfoHandling),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_LineInfoHandling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_LineInfoHandling", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
