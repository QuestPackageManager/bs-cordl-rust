#[cfg(feature = "UnityEngine+Bounds")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Bounds {
    pub m_Center: crate::UnityEngine::Vector3,
    pub m_Extents: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+Bounds")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bounds => "UnityEngine"."Bounds"
);
#[cfg(feature = "UnityEngine+Bounds")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Bounds {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Bounds")]
impl crate::UnityEngine::Bounds {
    pub fn ClosestPoint(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClosestPoint",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClosestPoint_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClosestPoint_Injected", (_unity_self, point, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains_Injected", (_unity_self, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encapsulate(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Encapsulate",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Bounds1(
        &mut self,
        other: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand(
        &mut self,
        amount: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Expand",
            (amount),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectRay(
        &mut self,
        ray: crate::UnityEngine::Ray,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntersectRay",
            (ray, distance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectRayAABB(
        ray: crate::UnityEngine::Ray,
        bounds: crate::UnityEngine::Bounds,
        dist: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectRayAABB", (ray, bounds, dist))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntersectRayAABB_Injected(
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        dist: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntersectRayAABB_Injected", (ray, bounds, dist))?;
        Ok(__cordl_ret.into())
    }
    pub fn Intersects(
        &mut self,
        bounds: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Intersects",
            (bounds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMinMax(
        &mut self,
        min: crate::UnityEngine::Vector3,
        max: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetMinMax",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        center: crate::UnityEngine::Vector3,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (center, _cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_center(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_center",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_extents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_extents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_max(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_max",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_min(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_min",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_size",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Bounds,
        rhs: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::Bounds,
        rhs: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_center(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_center",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_extents(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_extents",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_size",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Bounds")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Bounds>>
for crate::UnityEngine::Bounds {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Bounds> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Bounds")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Bounds>>
for crate::UnityEngine::Bounds {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Bounds> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Bounds")]
impl AsRef<crate::System::IFormattable> for crate::UnityEngine::Bounds {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Bounds")]
impl AsMut<crate::System::IFormattable> for crate::UnityEngine::Bounds {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
