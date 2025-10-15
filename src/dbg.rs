use crate::error::DbgError;
use std::{ffi::CString, path::PathBuf};
use windows::{
    Win32::System::Diagnostics::Debug::Extensions::*,
    core::{IUnknown, Interface, PCSTR},
};

/// Macro to send formatted messages to the debugger using [`Dbg::println`].
///
/// # Examples
///
/// ```rust,ignore
/// dprintln!(dbg, "Hello, {}!", "Debugger");
/// dprintln!(dbg, "This is a number: {}", 42);
/// ```
#[macro_export]
macro_rules! dprintln {
    ($dbg:expr) => {
        $dbg.println("");
    };

    ($dbg:expr, $($arg:tt)*) => {
        $dbg.println(format!($($arg)*));
    };
}

/// Macro to send formatted messages to the debugger using [`Dbg::print`]
///  
/// # Examples
///
/// ```rust,ignore
/// dprint!(dbg, "Hello, {}!", "Debugger");
/// dprint!(dbg, "This is a number: {}", 42);
/// dprint!(dbg); // Prints an empty message
/// ```
#[macro_export]
macro_rules! dprint {
    ($dbg:expr) => {
        $dbg.print("");
    };

    ($dbg:expr, $($arg:tt)*) => {
        $dbg.print(format!($($arg)*));
    };
}

/// Represents a debugging interface that allows execution of commands,
/// querying and managing debug symbols, inspecting memory, and interacting with registers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dbg {
    /// An interface to the debugging control system, allowing interactions with the debugger.
    pub control: IDebugControl3,

    /// An interface to manage and query debug symbols.
    pub symbols: IDebugSymbols3,

    /// An interface to access and manipulate memory spaces (data spaces) during debugging.
    pub dataspaces: IDebugDataSpaces4,

    /// An interface to query and manipulate CPU registers in the debugged target.
    pub registers: IDebugRegisters,
}

impl Dbg {
    /// Creates a new instance of the [`Dbg`] struct from a debugging client.
    ///
    /// # Arguments
    ///
    /// * `client` - A COM interface representing the debugging client.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = /* Obtain an instance of IDebugClient */;
    /// let dbg = Dbg::new(client)?;
    /// dbg.exec(".echo Hello World!")?;
    /// ```
    pub fn new(client: IUnknown) -> Result<Self, DbgError> {
        Ok(Self {
            control: client.cast()?,
            symbols: client.cast()?,
            dataspaces: client.cast()?,
            registers: client.cast()?,
        })
    }

    /// Creates a new debugger instance of the specified type.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = create_debug::<IDebugClient>()?;
    /// ```
    #[inline(always)]
    pub fn create_debug<T: Interface>() -> Result<T, DbgError> {
        unsafe { Ok(DebugCreate::<T>()?) }
    }

    /// Executes a command in the debugger.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute, provided as a string.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// dbg.exec(".echo Hello, Debugger!")?;
    /// ```
    pub fn exec<S>(&self, command: S) -> Result<(), DbgError>
    where
        S: Into<String>,
    {
        let cstr = CString::new(command.into())?;
        unsafe {
            Ok(self
                .control
                .Execute(DEBUG_OUTCTL_ALL_CLIENTS, PCSTR(cstr.as_ptr().cast()), DEBUG_EXECUTE_DEFAULT)?)
        }
    }

    /// Sends a message to the debugger output with a specific mask.
    ///
    /// # Arguments
    ///
    /// * `mask` - The output mask defining the message type (e.g., normal, error).
    /// * `str` - The message to send.
    fn output<S>(&self, mask: u32, str: S) -> Result<(), DbgError>
    where
        S: Into<String>,
    {
        let cstr = CString::new(str.into())?;
        unsafe { Ok(self.control.Output(mask, PCSTR(cstr.as_ptr().cast()))?) }
    }

    /// Logs a message to the debugger output.
    ///
    /// If the operation fails, an error message is sent to the debugger as a fallback.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// dbg.print("This is a normal log message.");
    /// ```
    pub fn print<S>(&self, args: S)
    where
        S: Into<String>,
    {
        if let Err(err) = self.output(DEBUG_OUTPUT_NORMAL, args) {
            // If it fails, it sends the error directly to the debugger as an error message.
            let _ = self.output(DEBUG_OUTPUT_ERROR, format!("Failed to log message: {:?}", err));
        }
    }

    /// Logs a message to the debugger output with a newline at the end.
    ///
    /// If the operation fails, an error message is sent to the debugger as a fallback.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// dbg.println("This is a normal log message.");
    /// ```
    pub fn println<S>(&self, args: S)
    where
        S: Into<String>,
    {
        if let Err(err) = self.output(DEBUG_OUTPUT_NORMAL, format!("{}\n", args.into())) {
            // If it fails, it sends the error directly to the debugger as an error message.
            let _ = self.output(DEBUG_OUTPUT_ERROR, format!("Failed to log message: {:?}", err));
        }
    }

    /// Retrieves the number of processors in the target system.
    #[inline(always)]
    pub fn num_processors(&self) -> Result<u32, DbgError> {
        unsafe { Ok(self.control.GetNumberProcessors()?) }
    }

    /// Retrieves the type of the debugged system.
    pub fn debug_type(&self) -> Result<(u32, u32), DbgError> {
        let (mut class, mut qualifier) = (0, 0);
        unsafe { self.control.GetDebuggeeType(&mut class, &mut qualifier)? };

        Ok((class, qualifier))
    }

    /// Evaluates an expression and returns the result as the specified type.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to be executed.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = dbg.eval::<u64>("0x200 + 0x300")?;
    /// ```
    pub fn eval<T>(&self, expr: &str) -> Result<T, DbgError>
    where
        T: DebugValue,
    {
        let cstr = CString::new(expr)?;
        let mut value = DEBUG_VALUE::default();
        unsafe {
            self.control
                .Evaluate(PCSTR(cstr.as_ptr().cast()), T::VALUE_TYPE, &mut value, None)?;
        }

        Ok(T::from_debug_value(&value))
    }

    /// Retrieves the address of a symbol by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// dbg.get_symbol_address("ntdll!NtAllocateVirtualMemory")?;
    /// ```
    pub fn get_symbol_address<S>(&self, name: S) -> Result<u64, DbgError>
    where
        S: Into<String>,
    {
        let cstr = CString::new(name.into())?;
        unsafe { Ok(self.symbols.GetOffsetByName(PCSTR(cstr.as_ptr().cast()))?) }
    }

    /// Resolves a symbol name from a given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to resolve to a symbol name.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// dbg.get_symbol_name(0x7FFF_FFFF_0000)?;
    /// ```
    pub fn get_symbol_name(&self, addr: u64) -> Result<String, DbgError> {
        // Allocate a buffer to hold the symbol name (initial size: 1024 bytes)
        let mut buffer = vec![0u8; 1024];
        let mut size = 0u32;
        unsafe {
            self.symbols.GetNameByOffset(addr, Some(&mut buffer), Some(&mut size), None)?;
        }

        // Check if the size of the symbol name is zero (indicating failure)
        if size == 0 {
            return Err(DbgError::InvalidSize(size as usize));
        }

        // Resize the buffer to match the actual size of the symbol name
        // Subtract 1 to exclude the null terminator added by the API
        buffer.resize((size - 1) as usize, 0);

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    /// Removes a synthetic module, either by its base address or by its name.
    ///
    /// # Arguments
    ///
    /// * `module` - A `Module` that represents either:
    ///   - `u64`: The base address of the module.
    ///   - `&str`: The name of the module.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// dbg.remove_synthetic_module(0x7FFF_FFFF_0000)?; // Remove by address
    /// dbg.remove_synthetic_module("example.dll")?; // Remove by name
    /// ```
    pub fn remove_synthetic_module<T>(&self, module: T) -> Result<(), DbgError>
    where
        T: Into<Module>,
    {
        match module.into() {
            Module::Address(base) => {
                // Remove the module by address
                unsafe { self.symbols.RemoveSyntheticModule(base)? }
            }
            Module::Name(name) => {
                let mut base = 0;
                let cstr = CString::new(name)?;
                unsafe {
                    // Resolve the address by name
                    self.symbols
                        .GetModuleByModuleName(PCSTR(cstr.as_ptr().cast()), 0, None, Some(&mut base))?;

                    // Remove the module by address
                    self.symbols.RemoveSyntheticModule(base)?
                }
            }
        }

        Ok(())
    }

    /// Reads a range of virtual memory into a buffer.
    ///
    /// # Arguments
    ///
    /// * `vaddr` - The starting virtual address to read from.
    /// * `buffer` - A mutable slice where the read bytes will be stored.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut buffer = vec![0u8; 1024]; // Prepare a buffer to hold the data
    /// let bytes_read = dbg.read_vaddr(0x7FFF_FFFF_0000, &mut buffer)?;
    /// ```
    pub fn read_vaddr(&self, vaddr: u64, buffer: &mut [u8]) -> Result<usize, DbgError> {
        let mut bytes_read = 0;
        unsafe {
            self.dataspaces
                .ReadVirtual(vaddr, buffer.as_mut_ptr().cast(), buffer.len() as u32, Some(&mut bytes_read))?;
        }

        Ok(bytes_read as usize)
    }

    /// Adds a synthetic module to the debugger's symbol table.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression used to compute the base address of the synthetic module.
    /// * `size` - The size of the synthetic module in bytes.
    /// * `name` - The name of the synthetic module. It must implement `Into<String>`.
    /// * `path` - The file path of the synthetic module. It must be a valid `PathBuf`.
    pub fn add_synthetic_module<S>(&self, expr: S, size: u32, name: S, path: PathBuf) -> Result<(), DbgError>
    where
        S: Into<String>,
    {
        // Compute the base address of the synthetic module
        // This evaluates the provided expression to a 64-bit address
        let base = self.eval::<u64>(&expr.into())?;

        // Convert the module name and image path into a CString
        let module_name = CString::new(name.into())?;
        let image_path = path
            .canonicalize()?
            .to_str()
            .ok_or(DbgError::DbgGeneralError("Invalid Image Path"))
            .and_then(|s| Ok(CString::new(s)?))?;

        unsafe {
            self.symbols.AddSyntheticModule(
                base,
                size,
                PCSTR(image_path.as_ptr().cast()),
                PCSTR(module_name.as_ptr().cast()),
                DEBUG_ADDSYNTHMOD_DEFAULT,
            )?;
        }

        Ok(())
    }

    /// Safely reads a value of a specified type from a given virtual memory address.
    ///
    /// # Arguments
    ///
    /// * `vaddr` - The starting virtual memory address to read from.
    pub fn read_type_vaddr<T: Copy>(&self, vaddr: u64) -> Result<T, DbgError> {
        // Determine the size of the target type `T`
        let size = size_of::<T>();
        if size == 0 {
            return Err(DbgError::InvalidSize(size));
        }

        // Prepare a buffer to hold the exact size of the target type
        let mut buffer = vec![0u8; size].into_boxed_slice();

        // Read the memory into the buffer
        self.read_vaddr(vaddr, &mut buffer)?;

        // SAFETY: `read_unaligned` ensures that we can handle unaligned memory safely
        let value = unsafe { (buffer.as_ptr() as *const T).read_unaligned() };

        Ok(value)
    }

    /// Reads the value of a specific Model-Specific Register (MSR).
    ///
    /// # Arguments
    ///
    /// * `msr` - The identifier of the MSR to read (usually a 32-bit value corresponding to a specific register).
    #[inline(always)]
    pub fn msr(&self, msr: u32) -> Result<u64, DbgError> {
        unsafe { Ok(self.dataspaces.ReadMsr(msr)?) }
    }

    /// Reads a null-terminated C string from a specific virtual memory address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The virtual memory address of the null-terminated string to read.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let str = dbg.read_cstr(0x7FFF_FFFF_0000)?;
    /// dprintln!(dbg, "String read: {str}");
    /// ```
    pub fn read_cstr(&self, addr: u64) -> Result<String, DbgError> {
        let max_bytes = 256;
        let mut buffer = vec![0u8; max_bytes];
        let mut size = 0;
        unsafe {
            self.dataspaces
                .ReadMultiByteStringVirtual(addr, max_bytes as u32, Some(&mut buffer), Some(&mut size))?;
        }

        // Check if the size of the symbol name is zero (indicating failure)
        if size == 0 {
            return Err(DbgError::InvalidSize(size as usize));
        }

        // Resize the buffer to match the actual size of the symbol name
        // Subtract 1 to exclude the null terminator added by the API
        buffer.resize((size - 1) as usize, 0);

        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    /// Retrieves the register indices corresponding to a provided list of names.
    ///
    /// # Arguments
    ///
    /// * `names` - A slice of string references (`&[&str]`) representing the names
    ///   of the registers for which the indices need to be fetched.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let regs = vec!["rax", "rbx", "rcx"];
    /// let indices = dbg.regs(&regs)?;
    /// ```
    pub fn reg_indices(&self, names: &[&str]) -> Result<Vec<u32>, DbgError> {
        names
            .iter()
            .map(|&n| {
                let name = CString::new(n)?;
                unsafe { Ok(self.registers.GetIndexByName(PCSTR(name.as_ptr().cast()))?) }
            })
            .collect()
    }

    /// Retrieves the values of the registers corresponding to a provided list of indices.
    ///
    /// # Arguments
    ///
    /// * `indices` - A slice of register indices (`&[u32]`) for which the values need to be fetched.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// let indices = vec![0, 1, 2]; // Assume these correspond to "rax", "rbx", "rcx".
    /// let values = dbg.reg_values(&indices)?; // Retrieve the values for the registers.
    /// ```
    pub fn reg_values(&self, indices: &[u32]) -> Result<Vec<DEBUG_VALUE>, DbgError> {
        let mut values = Vec::with_capacity(indices.len());
        unsafe {
            self.registers
                .GetValues(indices.len() as u32, Some(indices.as_ptr()), 0, values.as_mut_ptr())?;
        }

        Ok(values)
    }
}

/// A trait to extract a value from a [`DEBUG_VALUE`].
pub trait DebugValue: Sized {
    /// The corresponding `DEBUG_VALUE_TYPE` for this type.
    const VALUE_TYPE: u32;

    /// Extracts the value from a `DEBUG_VALUE`
    ///
    /// # Arguments
    ///
    /// * `val` - A reference to the `DEBUG_VALUE` to be converted.
    fn from_debug_value(val: &DEBUG_VALUE) -> Self;
}

impl DebugValue for u64 {
    /// Specifies that the value should be interpreted as a 64-bit integer.
    const VALUE_TYPE: u32 = DEBUG_VALUE_INT64;

    /// Extracts a 64-bit integer [`u64`] from a `DEBUG_VALUE`.
    fn from_debug_value(val: &DEBUG_VALUE) -> Self {
        unsafe { val.Anonymous.Anonymous.I64 }
    }
}

impl DebugValue for u32 {
    /// Specifies that the value should be interpreted as a 32-bit integer.
    const VALUE_TYPE: u32 = DEBUG_VALUE_INT32;

    /// Extracts a 32-bit integer ([`u32`] from a `DEBUG_VALUE`.
    fn from_debug_value(val: &DEBUG_VALUE) -> Self {
        unsafe { val.Anonymous.I32 }
    }
}

impl DebugValue for f64 {
    /// Specifies that the value should be interpreted as a 64-bit floating-point number.
    const VALUE_TYPE: u32 = DEBUG_VALUE_FLOAT64;

    /// Extracts a 64-bit floating-point value [`f64`] from a `DEBUG_VALUE`.
    fn from_debug_value(val: &DEBUG_VALUE) -> Self {
        unsafe { val.Anonymous.F64 }
    }
}

impl DebugValue for f32 {
    /// Specifies that the value should be interpreted as a 32-bit floating-point number.
    const VALUE_TYPE: u32 = DEBUG_VALUE_FLOAT32;

    /// Extracts a 32-bit floating-point value [`f32`] from a `DEBUG_VALUE`.
    fn from_debug_value(val: &DEBUG_VALUE) -> Self {
        unsafe { val.Anonymous.F32 }
    }
}

/// Represents either a module address or a module name.
///
/// This enum is used to pass arguments to methods like [`Dbg::remove_synthetic_module`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Module {
    /// A module identified by its base address.
    Address(u64),

    /// A module identified by its name.
    Name(String),
}

impl From<u64> for Module {
    /// Converts a [`u64`] into a [`Module::Address`].
    fn from(addr: u64) -> Self {
        Module::Address(addr)
    }
}

impl From<&str> for Module {
    /// Converts a `&str` into a [`Module::Name`].
    fn from(name: &str) -> Self {
        Module::Name(name.to_string())
    }
}
