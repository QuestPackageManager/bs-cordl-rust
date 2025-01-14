#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter_1<crate::UnityEngine::Color>,
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::JsonExtension::ColorConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.JsonExtension";
    const CLASS_NAME: &'static str = "ColorConverter";
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
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl std::ops::Deref for crate::BGLib::JsonExtension::ColorConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter_1<crate::UnityEngine::Color>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::ColorConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl crate::BGLib::JsonExtension::ColorConverter {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadJson(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        existingValue: crate::UnityEngine::Color,
        hasExistingValue: bool,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    crate::UnityEngine::Color,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
                ),
                crate::UnityEngine::Color,
                5usize,
            >("ReadJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadJson", 5usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (reader, objectType, existingValue, hasExistingValue, serializer),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: crate::UnityEngine::Color,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
                    crate::UnityEngine::Color,
                    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteJson")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteJson", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, value, serializer))
        };
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
}
#[cfg(feature = "BGLib+JsonExtension+ColorConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::JsonExtension::ColorConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
