#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct JTokenEqualityComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Linq";
    const CLASS_NAME: &'static str = "JTokenEqualityComparer";
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
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    pub fn Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        y: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
                ),
                bool,
                2usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>),
                i32,
                1usize,
            >("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (obj)) };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    >,
> for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenEqualityComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    >,
> for crate::Newtonsoft::Json::Linq::JTokenEqualityComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
