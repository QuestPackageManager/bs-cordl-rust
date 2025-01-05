#[cfg(feature = "UnityEngine+Ray2D")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Ray2D {
    pub m_Origin: crate::UnityEngine::Vector2,
    pub m_Direction: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+Ray2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Ray2D => "UnityEngine"."Ray2D"
);
#[cfg(feature = "UnityEngine+Ray2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Ray2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Ray2D")]
impl crate::UnityEngine::Ray2D {
    pub fn GetPoint(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPoint",
            (distance),
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
    pub fn ToString_Gc_Gc1(
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
        origin: crate::UnityEngine::Vector2,
        direction: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (origin, direction),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_direction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_origin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_origin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Ray2D")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::UnityEngine::Ray2D {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Ray2D")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::UnityEngine::Ray2D {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
