#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonObject {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonToken,
    pub _children: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Bson::BsonObject {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Bson";
    const CLASS_NAME: &'static str = "BsonObject";
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
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonObject {
    type Target = crate::Newtonsoft::Json::Bson::BsonToken;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl crate::Newtonsoft::Json::Bson::BsonObject {
    pub fn Add(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonToken>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, token))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerator_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Bson::BsonProperty,
                        >,
                    >,
                >,
                0usize,
            >("GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("System.Collections.IEnumerable.GetEnumerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Collections.IEnumerable.GetEnumerator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Newtonsoft::Json::Bson::BsonType,
                0usize,
            >("get_Type")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Type", 0usize
                )
            });
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
    >,
> for crate::Newtonsoft::Json::Bson::BsonObject {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
    >,
> for crate::Newtonsoft::Json::Bson::BsonObject {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Bson::BsonProperty>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Bson::BsonObject {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonObject")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Bson::BsonObject {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
