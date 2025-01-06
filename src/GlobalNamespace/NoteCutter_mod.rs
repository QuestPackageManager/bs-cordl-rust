#[cfg(feature = "NoteCutter")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colliders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        >,
    >,
    pub _cuttableBySaberSortParams: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams,
            >,
        >,
    >,
    pub _comparer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer,
    >,
}
#[cfg(feature = "NoteCutter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutter => ""."NoteCutter"
);
#[cfg(feature = "NoteCutter")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter")]
impl crate::GlobalNamespace::NoteCutter {
    pub const kMaxNumberOfColliders: i32 = 16i32;
    #[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
    pub type CuttableBySaberSortParams = crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams;
    #[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
    pub type CuttableBySaberSortParamsComparer = crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer;
    pub fn Cut(
        &mut self,
        saber: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Saber>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cut", (saber))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter_CuttableBySaberSortParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cuttableBySaber: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CuttableBySaber,
    >,
    pub distance: f32,
    pub pos: crate::UnityEngine::Vector3,
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteCutter_CuttableBySaberSortParams => ""
    ."NoteCutter/CuttableBySaberSortParams"
);
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutter_CuttableBySaberSortParamsComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer => ""
    ."NoteCutter/CuttableBySaberSortParamsComparer"
);
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl std::ops::Deref
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    pub fn Compare(
        &mut self,
        p0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        p1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (p0, p1))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl AsRef<crate::System::Collections::IComparer>
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteCutter+CuttableBySaberSortParamsComparer")]
impl AsMut<crate::System::Collections::IComparer>
for crate::GlobalNamespace::NoteCutter_CuttableBySaberSortParamsComparer {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
