#[cfg(feature = "UnityEngine+Playables+FrameRate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FrameRate {
    pub m_Rate: i32,
}
#[cfg(feature = "UnityEngine+Playables+FrameRate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::FrameRate =>
    "UnityEngine.Playables"."FrameRate"
);
#[cfg(feature = "UnityEngine+Playables+FrameRate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Playables::FrameRate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Playables+FrameRate")]
impl crate::UnityEngine::Playables::FrameRate {
    pub fn DoubleToFrameRate(
        framerate: f64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::FrameRate> {
        let __cordl_ret: crate::UnityEngine::Playables::FrameRate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoubleToFrameRate", (framerate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FrameRate0(
        &mut self,
        other: crate::UnityEngine::Playables::FrameRate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
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
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
            (),
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
        frameRate: u32,
        drop: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (frameRate, drop),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dropFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dropFrame",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rate(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::UnityEngine::Playables::FrameRate,
        b: crate::UnityEngine::Playables::FrameRate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+FrameRate")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::FrameRate>>
for crate::UnityEngine::Playables::FrameRate {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::FrameRate> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Playables+FrameRate")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::FrameRate>>
for crate::UnityEngine::Playables::FrameRate {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::FrameRate> {
        todo!()
    }
}
