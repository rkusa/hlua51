extern crate libc;
extern crate std;

use super::liblua;
use super::Index;
use super::Lua;
use super::Pushable;
use super::Readable;

macro_rules! integer_impl(
    ($t:ident) => (
        impl Pushable for $t {
            fn push_to_lua(&self, lua: &mut Lua) {
                unsafe { liblua::lua_pushinteger(lua.lua, *self as liblua::lua_Integer) }
            }
        }
        impl Readable for $t {
            fn read_from_lua(lua: &mut Lua, index: i32) -> Option<$t> {
                let success: libc::c_int = unsafe { std::mem::uninitialized() };
                let val = unsafe { liblua::lua_tointegerx(lua.lua, index, &success) };
                match success {
                    0 => None,
                    _ => Some(val as $t)
                }
            }
        }
        impl Index for $t {
        }
    );
)

integer_impl!(int)
integer_impl!(i8)
integer_impl!(i16)
integer_impl!(i32)
//integer_impl!(i64)   // data loss

macro_rules! unsigned_impl(
    ($t:ident) => (
        impl Pushable for $t {
            fn push_to_lua(&self, lua: &mut Lua) {
                unsafe { liblua::lua_pushunsigned(lua.lua, *self as liblua::lua_Unsigned) }
            }
        }
        impl Readable for $t {
            fn read_from_lua(lua: &mut Lua, index: i32) -> Option<$t> {
                let success: libc::c_int = unsafe { std::mem::uninitialized() };
                let val = unsafe { liblua::lua_tounsignedx(lua.lua, index, &success) };
                match success {
                    0 => None,
                    _ => Some(val as $t)
                }
            }
        }
        impl Index for $t {
        }
    );
)

unsigned_impl!(uint)
unsigned_impl!(u8)
unsigned_impl!(u16)
unsigned_impl!(u32)
//unsigned_impl!(u64)   // data loss

macro_rules! numeric_impl(
    ($t:ident) => (
        impl Pushable for $t {
            fn push_to_lua(&self, lua: &mut Lua) {
                unsafe { liblua::lua_pushnumber(lua.lua, *self as f64) }
            }
        }
        impl Readable for $t {
            fn read_from_lua(lua: &mut Lua, index: i32) -> Option<$t> {
                let success: libc::c_int = unsafe { std::mem::uninitialized() };
                let val = unsafe { liblua::lua_tonumberx(lua.lua, index, &success) };
                match success {
                    0 => None,
                    _ => Some(val as $t)
                }
            }
        }
        impl Index for $t {
        }
    );
)

numeric_impl!(f32)
numeric_impl!(f64)

impl Pushable for std::string::String {
    fn push_to_lua(&self, lua: &mut Lua) {
        unsafe { liblua::lua_pushstring(lua.lua, self.to_c_str().unwrap()) }
    }
}

impl Readable for String {
    fn read_from_lua(lua: &mut Lua, index: i32) -> Option<std::string::String> {
        let mut size: libc::size_t = unsafe { std::mem::uninitialized() };
        let cStrRaw = unsafe { liblua::lua_tolstring(lua.lua, index, &mut size) };
        if cStrRaw.is_null() {
            return None;
        }

        unsafe { std::c_str::CString::new(cStrRaw, false) }.as_str().map(|s| s.to_string())
    }
}

impl Index for String {
}

impl<'a> Pushable for &'a str {
    fn push_to_lua(&self, lua: &mut Lua) {
        unsafe { liblua::lua_pushstring(lua.lua, self.to_c_str().unwrap()) }
    }
}

impl Pushable for bool {
    fn push_to_lua(&self, lua: &mut Lua) {
        unsafe { liblua::lua_pushboolean(lua.lua, self.clone() as libc::c_int) }
    }
}

impl Readable for bool {
    fn read_from_lua(lua: &mut Lua, index: i32) -> Option<bool> {
        if unsafe { liblua::lua_isboolean(lua.lua, index) } != true {
            return None;
        }

        Some(unsafe { liblua::lua_toboolean(lua.lua, index) != 0 })
    }
}

impl Index for bool {
}

impl Pushable for () {
    fn push_to_lua(&self, lua: &mut Lua) {
        unsafe { liblua::lua_pushnil(lua.lua) }
    }
}

impl Readable for () {
    fn read_from_lua(lua: &mut Lua, index: i32) -> Option<()> {
        match unsafe { liblua::lua_isnil(lua.lua, index) } {
            true => Some(()),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_ints() {
        let mut lua = super::super::Lua::new();

        lua.set("a", 2).unwrap();

        let x: int = lua.get("a").unwrap();
        assert_eq!(x, 2);

        let y: i8 = lua.get("a").unwrap();
        assert_eq!(y, 2);

        let z: i16 = lua.get("a").unwrap();
        assert_eq!(z, 2);

        let w: i32 = lua.get("a").unwrap();
        assert_eq!(w, 2);

        let a: uint = lua.get("a").unwrap();
        assert_eq!(a, 2);

        let b: u8 = lua.get("a").unwrap();
        assert_eq!(b, 2);

        let c: u16 = lua.get("a").unwrap();
        assert_eq!(c, 2);

        let d: u32 = lua.get("a").unwrap();
        assert_eq!(d, 2);
    }

    #[test]
    fn write_ints() {
        // TODO: 

        let mut lua = super::super::Lua::new();

        lua.set("a", 2).unwrap();
        let x: int = lua.get("a").unwrap();
        assert_eq!(x, 2);
    }

    #[test]
    fn readwrite_floats() {
        let mut lua = super::super::Lua::new();

        lua.set("a", 2.51234 as f32).unwrap();
        lua.set("b", 3.4123456789 as f64).unwrap();

        let x: f32 = lua.get("a").unwrap();
        assert!(x - 2.51234 < 0.000001);

        let y: f64 = lua.get("a").unwrap();
        assert!(y - 2.51234 < 0.000001);

        let z: f32 = lua.get("b").unwrap();
        assert!(z - 3.4123456789 < 0.000001);

        let w: f64 = lua.get("b").unwrap();
        assert!(w - 3.4123456789 < 0.000001);
    }

    #[test]
    fn readwrite_bools() {
        let mut lua = super::super::Lua::new();

        lua.set("a", true).unwrap();
        lua.set("b", false).unwrap();

        let x: bool = lua.get("a").unwrap();
        assert_eq!(x, true);

        let y: bool = lua.get("b").unwrap();
        assert_eq!(y, false);
    }

    #[test]
    fn readwrite_strings() {
        let mut lua = super::super::Lua::new();

        lua.set("a", "hello").unwrap();
        lua.set("b", "hello".to_string()).unwrap();

        let x: String = lua.get("a").unwrap();
        assert_eq!(x.as_slice(), "hello");

        let y: String = lua.get("b").unwrap();
        assert_eq!(y.as_slice(), "hello");
    }

    #[test]
    fn int_to_string() {
        let mut lua = super::super::Lua::new();

        lua.set("a", 2).unwrap();

        let x: String = lua.get("a").unwrap();
        assert_eq!(x.as_slice(), "2");
    }

    #[test]
    fn string_to_int() {
        let mut lua = super::super::Lua::new();

        lua.set("a", "2").unwrap();
        lua.set("b", "aaa").unwrap();

        let x: int = lua.get("a").unwrap();
        assert_eq!(x, 2);

        let y: Option<int> = lua.get("b");
        assert!(y.is_none());
    }
}
