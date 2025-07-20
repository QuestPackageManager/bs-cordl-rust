#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct IWrappedDictionary {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "IWrappedDictionary";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_UnderlyingDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_UnderlyingDictionary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_UnderlyingDictionary", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsRef<crate::System::Collections::ICollection>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsMut<crate::System::Collections::ICollection>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsRef<crate::System::Collections::IDictionary>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_ref(&self) -> &crate::System::Collections::IDictionary {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsMut<crate::System::Collections::IDictionary>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IDictionary {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
