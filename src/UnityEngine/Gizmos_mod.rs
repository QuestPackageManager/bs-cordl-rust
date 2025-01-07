#[cfg(feature = "UnityEngine+Gizmos")]
#[repr(C)]
#[derive(Debug)]
pub struct Gizmos {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Gizmos")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Gizmos {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Gizmos";
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
#[cfg(feature = "UnityEngine+Gizmos")]
impl std::ops::Deref for crate::UnityEngine::Gizmos {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Gizmos")]
impl std::ops::DerefMut for crate::UnityEngine::Gizmos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Gizmos")]
impl crate::UnityEngine::Gizmos {
    pub fn DrawCube(
        center: crate::UnityEngine::Vector3,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawCube", (center, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawCube_Injected(
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawCube_Injected", (center, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawLine(
        from: crate::UnityEngine::Vector3,
        to: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawLine", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawLine_Injected(
        from: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        to: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawLine_Injected", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRay(
        from: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawRay", (from, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawSphere(
        center: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawSphere", (center, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawSphere_Injected(
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawSphere_Injected", (center, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireCube(
        center: crate::UnityEngine::Vector3,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawWireCube", (center, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireCube_Injected(
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawWireCube_Injected", (center, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireSphere(
        center: crate::UnityEngine::Vector3,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawWireSphere", (center, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireSphere_Injected(
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawWireSphere_Injected", (center, radius))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_color_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_matrix(
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_matrix", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_matrix_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_matrix_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Gizmos")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Gizmos {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
