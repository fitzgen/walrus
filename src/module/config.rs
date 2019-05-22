use crate::error::Result;
use crate::module::Module;
use std::path::Path;

/// Configuration for a `Module` which currently affects parsing.
#[derive(Clone, Debug, Default)]
pub struct ModuleConfig {
    pub(crate) generate_dwarf: bool,
    pub(crate) generate_synthetic_names_for_anonymous_items: bool,
    pub(crate) only_stable_features: bool,
    pub(crate) skip_strict_validate: bool,
    pub(crate) skip_producers_section: bool,
    pub(crate) skip_name_section: bool,
    pub(crate) preserve_code_transform: bool,
}

impl ModuleConfig {
    /// Creates a fresh new configuration with default settings.
    pub fn new() -> ModuleConfig {
        ModuleConfig::default()
    }

    /// Sets a flag to whether DWARF debug sections are generated for this
    /// module.
    ///
    /// By default this flag is `false`. Note that any emitted DWARF is
    /// currently wildly incorrect and buggy, and is also larger than the wasm
    /// itself!
    pub fn generate_dwarf(&mut self, generate: bool) -> &mut ModuleConfig {
        self.generate_dwarf = generate;
        self
    }

    /// Sets a flag to whether the custom "name" section is generated for this
    /// module.
    ///
    /// The "name" section contains symbol names for the module, functions, and
    /// locals. When enabled, stack traces will use these names, instead of
    /// `wasm-function[123]`.
    ///
    /// By default this flag is `true`.
    pub fn generate_name_section(&mut self, generate: bool) -> &mut ModuleConfig {
        self.skip_name_section = !generate;
        self
    }

    /// Sets a flag to whether synthetic debugging names are generated for
    /// anonymous locals/functions/etc when parsing and running passes for this
    /// module.
    ///
    /// By default this flag is `false`, and it will generate quite a few names
    /// if enabled!
    pub fn generate_synthetic_names_for_anonymous_items(
        &mut self,
        generate: bool,
    ) -> &mut ModuleConfig {
        self.generate_synthetic_names_for_anonymous_items = generate;
        self
    }

    /// Indicates whether the module, after parsing, performs strict validation
    /// of the wasm module to adhere with the current version of the wasm
    /// specification.
    ///
    /// This can be expensive for some modules and strictly isn't required to
    /// create a `Module` from a wasm file. This includes checks such as "atomic
    /// instructions require a shared memory".
    ///
    /// By default this flag is `true`
    pub fn strict_validate(&mut self, strict: bool) -> &mut ModuleConfig {
        self.skip_strict_validate = !strict;
        self
    }

    /// Indicates whether the module will have the "producers" custom section
    /// which preserves the original producers and also includes `walrus`.
    ///
    /// This is generally used for telemetry in browsers, but for otherwise tiny
    /// wasm binaries can add some size to the binary.
    ///
    /// By default this flag is `true`
    pub fn generate_producers_section(&mut self, generate: bool) -> &mut ModuleConfig {
        self.skip_producers_section = !generate;
        self
    }

    /// Indicates whether this module is allowed to use only stable WebAssembly
    /// features or not.
    ///
    /// This is currently used to disable some validity checks required by the
    /// WebAssembly specification. It's not religiously adhered to throughout
    /// the codebase, even if set to `true` some unstable features may still be
    /// allowed.
    ///
    /// By default this flag is `false`
    pub fn only_stable_features(&mut self, only: bool) -> &mut ModuleConfig {
        self.only_stable_features = only;
        self
    }

    /// Sets a flag to whether code transform is preverved during parsing.
    ///
    /// By default this flag is `false`.
    pub fn preserve_code_transform(&mut self, preserve: bool) -> &mut ModuleConfig {
        self.preserve_code_transform = preserve;
        self
    }

    /// Parses an in-memory WebAssembly file into a `Module` using this
    /// configuration.
    pub fn parse(&self, wasm: &[u8]) -> Result<Module> {
        Module::parse(wasm, self)
    }

    /// Parses a WebAssembly file into a `Module` using this configuration.
    pub fn parse_file<P>(&self, path: P) -> Result<Module>
    where
        P: AsRef<Path>,
    {
        Module::from_file_with_config(path, self)
    }
}
