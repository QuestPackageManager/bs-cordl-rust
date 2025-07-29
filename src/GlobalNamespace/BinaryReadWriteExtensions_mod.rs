#[cfg(feature = "cordl_class_BinaryReadWriteExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryReadWriteExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BinaryReadWriteExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BinaryReadWriteExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BinaryReadWriteExtensions";
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
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BinaryReadWriteExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BinaryReadWriteExtensions")]
impl crate::GlobalNamespace::BinaryReadWriteExtensions {
    pub fn ReadColor(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>),
                        crate::UnityEngine::Color,
                        1usize,
                    >("ReadColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadColor", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadListOf<T>(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        elementReader: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                                    T,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<T>,
                        >,
                        2usize,
                    >("ReadListOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadListOf", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryReader, elementReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPose(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>),
                        crate::UnityEngine::Pose,
                        1usize,
                    >("ReadPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadPose", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuaternion(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("ReadQuaternion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadQuaternion", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3(
        binaryReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("ReadVector3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadVector3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryReader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteListOf<T>(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        elementWriter: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<T>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                                    T,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteListOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteListOf", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryWriter, list, elementWriter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Color0(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryWriter, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Pose3(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        pose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                            crate::UnityEngine::Pose,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryWriter, pose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Quaternion2(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        quaternion: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                            crate::UnityEngine::Quaternion,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryWriter, quaternion))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Vector3_1(
        binaryWriter: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::BinaryWriter>,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (binaryWriter, vector))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BinaryReadWriteExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BinaryReadWriteExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
