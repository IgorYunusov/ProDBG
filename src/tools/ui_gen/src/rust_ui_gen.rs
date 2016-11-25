use std::io;
use std::fs::File;
use std::io::Write;

use ::data::*;

static HEADER: &'static [u8] = b"
use ffi_gen::*;
use widgets_gen::*;
pub struct Ui {
    wrui: *const Wrui
}\n
impl Ui {
    pub fn new(wrui: *const Wrui) -> Ui { Ui { wrui: wrui } }
    pub fn new_default() -> Ui { unsafe { Ui { wrui: wrui_get() } } }
    pub fn api_version(&self) -> u64 {
        unsafe {
            (*self.wrui).api_version
        }
    }\n\n";
///
/// Check if a entry is function ptr or note
///
fn is_create_func(func: &FuncPtr) -> bool {
	if func.name.find("_create").is_none() {
		return false;
	}

	func.return_val.is_some()
}

///
/// Generates the creation function in style like this:
///
///    pub fn application_create(&self) -> Application {
///        unsafe {
///            Application {
///                funcs: (*self.wrui).application_funcs,
///                app: ((*self.wrui).application_create)()
///            }
///        }
///    }
///
fn generate_create_func(f: &mut File, func_ptr: &FuncPtr) -> io::Result<()> {
	let type_name = func_ptr.return_val.as_ref().unwrap().rust_type.to_owned();
	let funcs_name = func_ptr.name.replace("create", "funcs"); 

    f.write_fmt(format_args!("    pub fn {}(&self) -> {} {{\n", func_ptr.name, type_name))?;
    f.write_fmt(format_args!("        unsafe {{\n"))?;
    f.write_fmt(format_args!("            {} {{\n", type_name))?;
    f.write_fmt(format_args!("                funcs: (*self.wrui).{},\n", funcs_name))?;
    f.write_fmt(format_args!("                obj: ((*self.wrui).{})()\n", func_ptr.name))?;
    f.write_all(b"           }\n")?;
    f.write_all(b"        }\n")?;
    f.write_all(b"    }\n\n")?;

    Ok(())
}

///
/// Generates the main entry point for the Ui functions
///
pub fn generate_ui(filename: &str, structs: &Vec<Struct>) -> io::Result<()> {
	let mut f = File::create(filename)?;

    f.write_all(HEADER)?;

	let wrui_struct = structs.iter().find(|&e| { e.name == "Wrui" } ).unwrap();

	for entry in &wrui_struct.entries {
		match entry {
			&StructEntry::FunctionPtr(ref func_ptr) => {
				if is_create_func(func_ptr) {
				    generate_create_func(&mut f, func_ptr)?;
				}
			}
			_ => (),
		}
	}

    f.write_all(b"}\n")?;

    Ok(())
}

