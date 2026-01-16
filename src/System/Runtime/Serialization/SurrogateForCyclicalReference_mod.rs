#[cfg(feature = "cordl_class_System+Runtime+Serialization+SurrogateForCyclicalReference")]
#[repr(C)]
#[derive(Debug)]
pub struct SurrogateForCyclicalReference {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub innerSurrogate:
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ISerializationSurrogate>,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+SurrogateForCyclicalReference")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::SurrogateForCyclicalReference
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "SurrogateForCyclicalReference";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl std::ops::Deref for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl crate::System::Runtime::Serialization::SurrogateForCyclicalReference {
    pub fn GetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::SerializationInfo>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::SerializationInfo,
                        >,
                        crate::System::Runtime::Serialization::StreamingContext,
                    ), quest_hook::libil2cpp::Void, 3usize>("GetObjectData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetObjectData",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (obj, info, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::SerializationInfo>,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::SerializationInfo,
                        >,
                        crate::System::Runtime::Serialization::StreamingContext,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ISurrogateSelector,
                        >,
                    ), quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, 4usize>(
                        "SetObjectData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetObjectData",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (obj, info, context, selector))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::SurrogateForCyclicalReference
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl AsRef<crate::System::Runtime::Serialization::ISerializationSurrogate>
    for crate::System::Runtime::Serialization::SurrogateForCyclicalReference
{
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SurrogateForCyclicalReference")]
impl AsMut<crate::System::Runtime::Serialization::ISerializationSurrogate>
    for crate::System::Runtime::Serialization::SurrogateForCyclicalReference
{
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializationSurrogate {
        unsafe { std::mem::transmute(self) }
    }
}
