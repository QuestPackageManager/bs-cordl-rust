#[cfg(feature = "PosesRecordingSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub objectIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub keyframes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
            >,
        >,
    >,
    pub externalCameraCalibration: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
    >,
}
#[cfg(feature = "PosesRecordingSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingSaveData";
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
#[cfg(feature = "PosesRecordingSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesRecordingSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl crate::GlobalNamespace::PosesRecordingSaveData {
    #[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
    pub type ExternalCameraCalibrationSaveData = crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData;
    #[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
    pub type PoseSaveData = crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData;
    #[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
    pub type TransformsSaveKeyframe = crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe;
    pub fn New(
        objectIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        keyframes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
                >,
            >,
        >,
        externalCameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectIds, keyframes, externalCameraCalibration))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        objectIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        keyframes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
                >,
            >,
        >,
        externalCameraCalibration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (objectIds, keyframes, externalCameraCalibration),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub fieldOfVision: f32,
    pub nearClip: f32,
    pub farClip: f32,
    pub hmdOffset: f32,
    pub nearOffset: f32,
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingSaveData/ExternalCameraCalibrationSaveData";
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
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    pub fn New(
        fieldOfVision: f32,
        nearClip: f32,
        farClip: f32,
        hmdOffset: f32,
        nearOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (fieldOfVision, nearClip, farClip, hmdOffset, nearOffset),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fieldOfVision: f32,
        nearClip: f32,
        farClip: f32,
        hmdOffset: f32,
        nearOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32, f32, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (fieldOfVision, nearClip, farClip, hmdOffset, nearOffset),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingSaveData+ExternalCameraCalibrationSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_ExternalCameraCalibrationSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_PoseSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub posX: f32,
    pub posY: f32,
    pub posZ: f32,
    pub rotX: f32,
    pub rotY: f32,
    pub rotZ: f32,
    pub rotW: f32,
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingSaveData/PoseSaveData";
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
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    pub fn New(
        posX: f32,
        posY: f32,
        posZ: f32,
        rotX: f32,
        rotY: f32,
        rotZ: f32,
        rotW: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (posX, posY, posZ, rotX, rotY, rotZ, rotW))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        posX: f32,
        posY: f32,
        posZ: f32,
        rotX: f32,
        rotY: f32,
        rotZ: f32,
        rotW: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32, f32, f32, f32, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (posX, posY, posZ, rotX, rotY, rotZ, rotW))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingSaveData+PoseSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
#[repr(C)]
#[derive(Debug)]
pub struct PosesRecordingSaveData_TransformsSaveKeyframe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub poses: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
            >,
        >,
    >,
    pub _cordl_time: f32,
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosesRecordingSaveData/TransformsSaveKeyframe";
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
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl std::ops::Deref
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    pub fn New(
        poses: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
                >,
            >,
        >,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (poses, _cordl_time))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        poses: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
                >,
            >,
        >,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::PosesRecordingSaveData_PoseSaveData,
                                    >,
                                >,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (poses, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosesRecordingSaveData+TransformsSaveKeyframe")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PosesRecordingSaveData_TransformsSaveKeyframe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
