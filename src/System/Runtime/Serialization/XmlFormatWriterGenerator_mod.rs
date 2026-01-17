#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct XmlFormatWriterGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub helper: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper,
    >,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlFormatWriterGenerator";
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
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator")]
impl std::ops::Deref for crate::System::Runtime::Serialization::XmlFormatWriterGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::XmlFormatWriterGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator")]
impl crate::System::Runtime::Serialization::XmlFormatWriterGenerator {
    #[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
    pub type CriticalHelper =
        crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper;
    pub fn GenerateClassWriter(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::ClassDataContract,
                    >), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
                    >, 1usize>("GenerateClassWriter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateClassWriter",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (classContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCollectionWriter(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::CollectionDataContract,
                    >), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
                    >, 1usize>("GenerateCollectionWriter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateCollectionWriter",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (collectionContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct XmlFormatWriterGenerator_CriticalHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlFormatWriterGenerator/CriticalHelper";
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
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
impl std::ops::Deref
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
impl std::ops::DerefMut
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
impl crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper {
    pub fn GenerateClassWriter(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::ClassDataContract,
                    >), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
                    >, 1usize>("GenerateClassWriter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateClassWriter",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassWriterDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (classContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCollectionWriter(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::CollectionDataContract,
                    >), quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
                    >, 1usize>("GenerateCollectionWriter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateCollectionWriter",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionWriterDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (collectionContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterGenerator+CriticalHelper")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::XmlFormatWriterGenerator_CriticalHelper
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
