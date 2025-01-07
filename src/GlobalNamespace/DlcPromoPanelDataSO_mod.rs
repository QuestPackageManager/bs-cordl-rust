#[cfg(feature = "DlcPromoPanelDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelDataSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _cutOffTest: i32,
    pub _minNumberOfNotOwnedPacks: i32,
    pub _defaultPromoInfoId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _dlcPromoPanelType: crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType,
    pub _customDlcPromoBanner: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PromoBannerInfoSO,
    >,
}
#[cfg(feature = "DlcPromoPanelDataSO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::DlcPromoPanelDataSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DlcPromoPanelDataSO";
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
#[cfg(feature = "DlcPromoPanelDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelDataSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::DlcPromoPanelDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl crate::GlobalNamespace::DlcPromoPanelDataSO {
    #[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
    pub type DlcPromoPanelType = crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType;
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
    pub fn get_customDlcPromoBanner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PromoBannerInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        > = __cordl_object.invoke("get_customDlcPromoBanner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutOffTest(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cutOffTest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultPromoInfoId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_defaultPromoInfoId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dlcPromoPanelType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType = __cordl_object
            .invoke("get_dlcPromoPanelType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minNumberOfNotOwnedPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_minNumberOfNotOwnedPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultPromoInfoId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultPromoInfoId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DlcPromoPanelDataSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DlcPromoPanelDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DlcPromoPanelDataSO_DlcPromoPanelType {
    #[default]
    Pack = 0i32,
    Store = 1i32,
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DlcPromoPanelType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "DlcPromoPanelDataSO+DlcPromoPanelType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::DlcPromoPanelDataSO_DlcPromoPanelType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
