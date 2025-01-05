#[cfg(feature = "SafeAreaRectChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeAreaRectChecker {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _minAngleX: f32,
    pub _maxAngleX: f32,
    pub _minAngleY: f32,
    pub _maxAngleY: f32,
    pub _activeObjectWhenInsideSafeArea: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _activeObjectWhenNotInsideSafeArea: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _rectTransformToCheck: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub _corners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub _mainCamera: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainCamera>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SafeAreaRectChecker_InitData,
    >,
}
#[cfg(feature = "SafeAreaRectChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SafeAreaRectChecker => ""
    ."SafeAreaRectChecker"
);
#[cfg(feature = "SafeAreaRectChecker")]
impl std::ops::Deref for crate::GlobalNamespace::SafeAreaRectChecker {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SafeAreaRectChecker")]
impl std::ops::DerefMut for crate::GlobalNamespace::SafeAreaRectChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SafeAreaRectChecker")]
impl crate::GlobalNamespace::SafeAreaRectChecker {
    #[cfg(feature = "SafeAreaRectChecker+InitData")]
    pub type InitData = crate::GlobalNamespace::SafeAreaRectChecker_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SafeAreaRectChecker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SafeAreaRectChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SafeAreaRectChecker+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeAreaRectChecker_InitData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub checkingEnabled: bool,
}
#[cfg(feature = "SafeAreaRectChecker+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SafeAreaRectChecker_InitData =>
    ""."SafeAreaRectChecker/InitData"
);
#[cfg(feature = "SafeAreaRectChecker+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::SafeAreaRectChecker_InitData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SafeAreaRectChecker+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SafeAreaRectChecker_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SafeAreaRectChecker+InitData")]
impl crate::GlobalNamespace::SafeAreaRectChecker_InitData {
    pub fn New(
        checkingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (checkingEnabled))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        checkingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (checkingEnabled))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SafeAreaRectChecker+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SafeAreaRectChecker_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
