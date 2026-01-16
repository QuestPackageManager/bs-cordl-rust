#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlFormatReaderGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub helper: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper,
    >,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlFormatReaderGenerator";
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
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator")]
impl std::ops::Deref for crate::System::Runtime::Serialization::XmlFormatReaderGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::XmlFormatReaderGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator")]
impl crate::System::Runtime::Serialization::XmlFormatReaderGenerator {
    #[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
    pub type CriticalHelper =
        crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper;
    pub fn GenerateClassReader(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
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
                        crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
                    >, 1usize>("GenerateClassReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateClassReader",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (classContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCollectionReader(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
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
                        crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
                    >, 1usize>("GenerateCollectionReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateCollectionReader",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (collectionContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGetOnlyCollectionReader(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataContract,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
                        >,
                        1usize,
                    >("GenerateGetOnlyCollectionReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateGetOnlyCollectionReader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
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
    pub fn UnsafeGetUninitializedObject(
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("UnsafeGetUninitializedObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnsafeGetUninitializedObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlFormatReaderGenerator_CriticalHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlFormatReaderGenerator/CriticalHelper";
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
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
impl std::ops::Deref
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
impl std::ops::DerefMut
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
impl crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper {
    pub fn GenerateClassReader(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
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
                        crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
                    >, 1usize>("GenerateClassReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateClassReader",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatClassReaderDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (classContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCollectionReader(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
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
                        crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
                    >, 1usize>("GenerateCollectionReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateCollectionReader",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatCollectionReaderDelegate,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (collectionContract))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGetOnlyCollectionReader(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataContract,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
                        >,
                        1usize,
                    >("GenerateGetOnlyCollectionReader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateGetOnlyCollectionReader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlFormatGetOnlyCollectionReaderDelegate,
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
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatReaderGenerator+CriticalHelper")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Runtime::Serialization::XmlFormatReaderGenerator_CriticalHelper
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
