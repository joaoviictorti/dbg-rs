use dbg_rs::{dprintln, error::DbgError, Dbg};
use windows::{
    core::{IUnknown, Interface, HRESULT},
    Win32::{
        Foundation::{E_ABORT, S_OK},
        System::Diagnostics::Debug::Extensions::IDebugClient,
    },
};

/// List loaded modules, skipping unloaded ones.
fn wrap(dbg: &Dbg) -> Result<(), DbgError> {
    let mut index = 0;

    // Iterate over all modules in the process.
    loop {
        // Retrieve the base address of the module.
        let base = match unsafe { dbg.symbols.GetModuleByIndex(index) } {
            Ok(base) => base,
            Err(_) => break, // Stop when no more modules are found.
        };

        // Get the module name from its base address.
        let module_name = dbg.get_symbol_name(base).unwrap_or_else(|_| "<unknown>".to_string());

        // Skip unloaded modules.
        if module_name.contains("<Unloaded_") {
            index += 1;
            continue;
        }

        // Clean and print the module name.
        let clean_name = module_name.split('!').next().unwrap_or(&module_name).trim();
        dprintln!(dbg, "[dbg] Module {index}: {clean_name} (Base: {base:#X})");

        index += 1;
    }

    dprintln!(dbg, "[dbg] Finished listing modules.");
    Ok(())
}

/// Entry point for the command `!list_modules`.
#[unsafe(no_mangle)]
extern "C" fn list_modules(client: *mut IDebugClient, _args: *const u8) -> HRESULT {
    let client = unsafe { IUnknown::from_raw(client.cast()) };
    let Ok(dbg) = Dbg::new(client) else {
        return E_ABORT;
    };

    match wrap(&dbg) {
        Ok(_) => S_OK,
        Err(err) => {
            dprintln!(dbg, "[dbg] Error: {err}");
            E_ABORT
        }
    }
}

/// Initialize the extension.
#[unsafe(export_name = "DebugExtensionInitialize")]
extern "C" fn init() -> HRESULT {
    S_OK
}

/// Uninitialize the extension.
#[unsafe(export_name = "DebugExtensionUninitialize")]
extern "C" fn uinit() {}
